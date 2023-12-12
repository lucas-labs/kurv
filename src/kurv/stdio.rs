use {
    anyhow::Result,
    std::fs::File,
    std::path::{Path, PathBuf},
    log::error
};

use crate::common::error::Error;

/// The type of an stdio file.
enum StdioFile {
    Stdout,
    Stderr,
}

/// Create and return the two file handles for the `(stdout, stderr)` log file of a task.
/// These are two handles to the same file.
pub fn create_log_file_handles(task_name: &String, path: &Path) -> Result<(File, File), Error> {
    let (stdout_path, stderr_path) = get_log_paths(task_name, path);

    let stdout_handle = File::create(&stdout_path)
        .map_err(|err| Error::IoPathError(stdout_path.clone(), "getting stdout handle", err))?;

    let stderr_handle = File::create(&stderr_path)
        .map_err(|err| Error::IoPathError(stderr_path.clone(), "getting stderr handle", err))?;

    Ok((stdout_handle, stderr_handle))
}

/// Get the path to the log file of a task.
pub fn get_log_paths(task_name: &String, path: &Path) -> (PathBuf, PathBuf) {
    let task_log_dir = path.join("task_logs");

    (
        task_log_dir.join(stdio_filename(task_name, StdioFile::Stdout)),
        task_log_dir.join(stdio_filename(task_name, StdioFile::Stderr)),
    )
}

/// Get the filename of the log file of a task.
fn stdio_filename(task_name: &String, file_type: StdioFile) -> String {
    // make task_name kebab-case
    task_name.clone()
        + match file_type {
            StdioFile::Stdout => ".stdout",
            StdioFile::Stderr => ".stderr",
        }
}

/// Remove the the log files of a task.
pub fn clean_log_handles(task_name: &String, path: &Path) {
    let (stdout_path, stderr_path) = get_log_paths(task_name, path);

    if stdout_path.exists() {
        if let Err(err) = std::fs::remove_file(stdout_path) {
            error!("Failed to remove stdout file for task {task_name} with error {err:?}");
        };
    }

    if stderr_path.exists() {
        if let Err(err) = std::fs::remove_file(stderr_path) {
            error!("Failed to remove stderr file for task {task_name} with error {err:?}");
        };
    }
}