use {
    anyhow::{Result, anyhow},
    log::error,
    std::{
        fs::{File, OpenOptions, create_dir_all},
        path::{Path, PathBuf},
    },
};

/// The type of an stdio file.
enum StdioFile {
    Stdout,
    Stderr,
}

/// Create and return the two file handles for the `(stdout, stderr)` log file of a task.
/// These are two handles to the same file.
pub fn create_log_file_handles(
    task_name: &str,
    path: &Path,
) -> Result<((PathBuf, File), (PathBuf, File))> {
    let (stdout_path, stderr_path) = get_log_paths(task_name, path);
    let stdout_handle = create_or_append_file(&stdout_path)?;
    let stderr_handle = create_or_append_file(&stderr_path)?;

    Ok(((stdout_path, stdout_handle), (stderr_path, stderr_handle)))
}

/// creates a file or opens it for appending if it already exists
fn create_or_append_file(path: &Path) -> Result<File> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent).map_err(|err| anyhow!("failed to create directories: {}", err))?;
    }

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|err| anyhow!("getting stdout handle: {}", err))
}

/// Get the path to the log file of a task.
pub fn get_log_paths(task_name: &str, path: &Path) -> (PathBuf, PathBuf) {
    (
        path.join(stdio_filename(task_name, StdioFile::Stdout)),
        path.join(stdio_filename(task_name, StdioFile::Stderr)),
    )
}

/// Get the filename of the log file of a task.
fn stdio_filename(task_name: &str, file_type: StdioFile) -> String {
    // make task_name kebab-case
    task_name.to_owned().clone()
        + match file_type {
            StdioFile::Stdout => ".stdout",
            StdioFile::Stderr => ".stderr",
        }
}

/// Remove the the log files of a task.
pub fn clean_log_handles(task_name: &String, path: &Path) {
    let (stdout_path, stderr_path) = get_log_paths(task_name, path);

    if stdout_path.exists()
        && let Err(err) = std::fs::remove_file(stdout_path)
    {
        error!("Failed to remove stdout file for task {task_name} with error {err:?}");
    };

    if stderr_path.exists()
        && let Err(err) = std::fs::remove_file(stderr_path)
    {
        error!("Failed to remove stderr file for task {task_name} with error {err:?}");
    };
}
