use {
    crate::{
        app::kurv_ui::KurvAppContext,
        db::{
            migrations::{Migrator, MigratorTrait},
            models::user::Entity as User,
        },
    },
    axum::{
        extract::{FromRef, FromRequestParts},
        http::request::Parts,
    },
    eyre::Result,
    sea_orm::{EntityTrait, PaginatorTrait},
    std::fmt::Display,
};

pub struct SetupService {
    pub db: sea_orm::DatabaseConnection,
}

impl FromRequestParts<KurvAppContext> for SetupService {
    type Rejection = String;
    async fn from_request_parts(
        _: &mut Parts,
        state: &KurvAppContext,
    ) -> Result<Self, Self::Rejection> {
        let db = KurvAppContext::from_ref(state).db;
        Ok(SetupService { db })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetupStatus {
    Uninitialized,
    Ready,
}

impl Display for SetupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupStatus::Uninitialized => write!(f, "uninitialized"),
            SetupStatus::Ready => write!(f, "ready"),
        }
    }
}

impl SetupService {
    /// Returns the current setup status of the application.
    /// 1. `Uninitialized`: Initial user has not been created.
    /// 2. `Ready`: Initial user has been created. Setup is complete.
    pub async fn get_status(&self) -> Result<SetupStatus> {
        let selection = User::find();
        let user_count = selection.count(&self.db).await?;

        if user_count == 0 {
            Ok(SetupStatus::Uninitialized)
        } else {
            Ok(SetupStatus::Ready)
        }
    }

    /// Run Sea-ORM migrations to ensure the database schema is up-to-date.
    pub async fn migrate(&self) -> Result<()> {
        Migrator::up(&self.db, None).await?;
        Ok(())
    }
}
