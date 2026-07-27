use {
    crate::{
        app::kurv_ui::KurvAppContext,
        common::err::{self, AppErr},
    },
    axum::{
        extract::{FromRef, FromRequestParts},
        http::{StatusCode, request::Parts},
    },
    chrono::{DateTime, Local},
    serde::{Deserialize, Serialize, de::DeserializeOwned},
    std::{
        collections::HashMap,
        io::{Read, Write},
        net::TcpStream,
        path::PathBuf,
        str,
    },
    tokio::task,
};

pub struct KurvApiService {
    host: String,
    port: u16,
}

impl FromRequestParts<KurvAppContext> for KurvApiService {
    type Rejection = String;

    async fn from_request_parts(
        _: &mut Parts,
        state: &KurvAppContext,
    ) -> Result<Self, Self::Rejection> {
        let env = KurvAppContext::from_ref(state).env;

        Ok(Self {
            host: env.api_host,
            port: env.api_port,
        })
    }
}

#[derive(Deserialize)]
struct KurvErrorResponse {
    message: String,
}

#[derive(Debug)]
struct KurvResponse {
    status: u16,
    body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KurvEggStatus {
    Pending,
    Running,
    Stopped,
    PendingRemoval,
    Restarting,
    Errored,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KurvEggState {
    pub status: KurvEggStatus,
    pub start_time: Option<DateTime<Local>>,
    pub try_count: u32,
    pub error: Option<String>,
    pub pid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KurvEggPaths {
    pub stdout: PathBuf,
    pub stderr: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KurvEgg {
    pub command: String,
    pub name: String,
    pub id: Option<usize>,
    pub state: Option<KurvEggState>,
    pub args: Option<Vec<String>>,
    pub cwd: Option<PathBuf>,
    pub env: Option<HashMap<String, String>>,
    pub paths: Option<KurvEggPaths>,
    pub plugin: Option<bool>,
    pub plugin_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KurvEggSummary {
    pub id: usize,
    pub pid: u32,
    pub name: String,
    pub status: KurvEggStatus,
    pub uptime: String,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KurvEggSummaryList(pub Vec<KurvEggSummary>);

impl KurvApiService {
    pub async fn list_eggs(&self, kind: Option<&str>) -> Result<KurvEggSummaryList, AppErr> {
        let path = match kind {
            Some("plugins") => "/eggs?kind=plugins".to_string(),
            _ => "/eggs?kind=eggs".to_string(),
        };

        self.request("GET", path, None).await
    }

    pub async fn get_egg(&self, egg_id: &str) -> Result<KurvEgg, AppErr> {
        self.request("GET", format!("/eggs/{egg_id}"), None).await
    }

    pub async fn start_egg(&self, egg_id: &str) -> Result<KurvEgg, AppErr> {
        self.request("POST", format!("/eggs/{egg_id}/start"), Some(String::new())).await
    }

    pub async fn stop_egg(&self, egg_id: &str) -> Result<KurvEgg, AppErr> {
        self.request("POST", format!("/eggs/{egg_id}/stop"), Some(String::new())).await
    }

    pub async fn restart_egg(&self, egg_id: &str) -> Result<KurvEgg, AppErr> {
        self.request("POST", format!("/eggs/{egg_id}/restart"), Some(String::new())).await
    }

    async fn request<T>(
        &self,
        method: &'static str,
        path: String,
        body: Option<String>,
    ) -> Result<T, AppErr>
    where
        T: DeserializeOwned,
    {
        let host = self.host.clone();
        let port = self.port;

        let response = task::spawn_blocking(move || request_sync(method, &host, port, &path, body))
            .await
            .map_err(|error| err::internal_error(format!("kurv request task failed: {error}")))?
            .map_err(|error| {
                err::err(StatusCode::BAD_GATEWAY, format!("failed to reach kurv server: {error}"))
            })?;

        parse_response(response)
    }
}

fn request_sync(
    method: &str,
    host: &str,
    port: u16,
    path: &str,
    body: Option<String>,
) -> Result<KurvResponse, String> {
    let mut stream = TcpStream::connect(format!("{host}:{port}"))
        .map_err(|error| format!("connect error: {error}"))?;

    let body_section = match body {
        Some(body) => format!("Content-Length: {}\r\n\r\n{}", body.len(), body),
        None => String::from("\r\n"),
    };

    let request = format!("{method} {path} HTTP/1.1\r\nHost: {host}\r\n{body_section}\r\n");

    stream.write_all(request.as_bytes()).map_err(|error| format!("write error: {error}"))?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).map_err(|error| format!("read error: {error}"))?;

    let response = str::from_utf8(&buffer).map_err(|error| format!("utf8 error: {error}"))?;
    let mut parts = response.splitn(2, "\r\n\r\n");
    let headers = parts.next().unwrap_or_default();
    let body = parts.next().unwrap_or_default().to_string();

    let status = headers
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|status| status.parse::<u16>().ok())
        .ok_or_else(|| "invalid response status".to_string())?;

    Ok(KurvResponse { status, body })
}

fn parse_response<T>(response: KurvResponse) -> Result<T, AppErr>
where
    T: DeserializeOwned,
{
    let status = StatusCode::from_u16(response.status).unwrap_or(StatusCode::BAD_GATEWAY);

    if !status.is_success() {
        let message = serde_json::from_str::<KurvErrorResponse>(&response.body)
            .map(|error| error.message)
            .unwrap_or_else(|_| {
                let body = response.body.trim();

                if body.is_empty() {
                    format!("kurv server returned {}", status.as_u16())
                } else {
                    body.to_string()
                }
            });

        return Err(err::err(status, message));
    }

    serde_json::from_str(&response.body)
        .map_err(|error| err::internal_error(format!("failed to parse kurv response: {error}")))
}
