use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unexpected I/O error:\n{}", .0)]
    RawIoError(#[from] std::io::Error),

    #[error("I/O error at path {:?} while {}:\n{}", .0, .1, .2)]
    IoPathError(PathBuf, &'static str, std::io::Error),
}
