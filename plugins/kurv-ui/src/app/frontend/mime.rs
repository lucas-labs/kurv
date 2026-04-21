use std::{ffi::OsStr, path::Path};

/// A mapping of common file extensions and their MIME types.
pub static MIME_TYPES: &[(&str, &[&str])] = &[
    ("", &["application/octet-stream"]),
    ("css", &["text/css"]),
    ("html", &["text/html"]),
    ("jpg", &["image/jpeg"]),
    ("js", &["text/javascript"]),
    ("json", &["application/json"]),
    ("png", &["image/png"]),
    ("svg", &["image/svg+xml"]),
    ("webm", &["video/webm"]),
    ("webp", &["image/webp"]),
    ("woff", &["font/woff", "application/font-woff"]),
    ("woff2", &["font/woff2"]),
];

pub fn guess<P: AsRef<Path>>(path: P) -> &'static str {
    let extension =
        path.as_ref().extension().and_then(OsStr::to_str).unwrap_or_default().to_lowercase();

    map_lookup(MIME_TYPES, &extension)
        .map(|mime_types| mime_types[0])
        .unwrap_or("application/octet-stream")
}

fn map_lookup<V>(map: &'static [(&'static str, V)], key: &str) -> Option<V>
where
    V: Copy,
{
    map.binary_search_by_key(&key, |(k, _)| *k).ok().map(|i| map[i].1)
}
