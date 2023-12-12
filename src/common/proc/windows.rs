// We allow anyhow in here, as this is a module that'll be strictly used internally.
// As soon as it's obvious that this is code is intended to be exposed to library users, we have to
// go ahead and replace any `anyhow` usage by proper error handling via our own Error type.
use anyhow::{bail, Result};
use command_group::GroupChild;
use winapi::shared::minwindef::FALSE;
use winapi::shared::ntdef::NULL;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::processthreadsapi::{OpenThread, ResumeThread, SuspendThread};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, Thread32First, Thread32Next,
    PROCESSENTRY32, TH32CS_SNAPPROCESS, TH32CS_SNAPTHREAD, THREADENTRY32,
};
use winapi::um::winnt::THREAD_SUSPEND_RESUME;

/// Shim signal enum for windows.
pub enum Signal {
    SIGINT,
    SIGKILL,
    SIGTERM,
    SIGCONT,
    SIGSTOP,
}

/// Send a signal to a windows process.
pub fn send_signal_to_child<T>(child: &mut GroupChild, signal: T) -> Result<()>
where
    T: Into<Signal>,
{
    let pids = get_cur_task_processes(child.id());
    if pids.is_empty() {
        bail!("Process has just gone away");
    }

    let signal: Signal = signal.into();

    match signal {
        Signal::SIGSTOP => {
            for pid in pids {
                for thread in get_threads(pid) {
                    suspend_thread(thread);
                }
            }
        }
        Signal::SIGCONT => {
            for pid in pids {
                for thread in get_threads(pid) {
                    resume_thread(thread);
                }
            }
        }
        _ => {
            bail!("Trying to send unix signal on a windows machine. This isn't supported.");
        }
    }

    Ok(())
}

/// Kill a child process
pub fn kill_child(task_id: usize, child: &mut GroupChild) -> std::io::Result<()> {
    match child.kill() {
        Ok(_) => Ok(()),
        Err(ref e) if e.kind() == std::io::ErrorKind::InvalidData => {
            // Process already exited
            // info!("Task {task_id} has already finished by itself.");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

/// Get current task pid, all child pid and all children's children
/// TODO: see if this can be simplified using QueryInformationJobObject
/// on the job object created by command_group.
fn get_cur_task_processes(task_pid: u32) -> Vec<u32> {
    let mut all_pids = Vec::new();

    // Get all pids by BFS
    let mut parent_pids = vec![task_pid];
    while let Some(pid) = parent_pids.pop() {
        all_pids.push(pid);

        get_child_pids(pid, &mut parent_pids);
    }

    // Keep parent pid ahead of child. We need execute action for parent process first.
    all_pids.reverse();
    all_pids
}

/// Get child pids of a specific process.
fn get_child_pids(target_pid: u32, pid_list: &mut Vec<u32>) {
    unsafe {
        // Take a snapshot of all processes in the system.
        // While enumerating the set of processes, new processes can be created and destroyed.
        let snapshot_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, target_pid);
        if snapshot_handle == INVALID_HANDLE_VALUE {
            // error!("Failed to get process {target_pid} snapShot");
            return;
        }

        // Walk the list of processes.
        let mut process_entry = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };
        if Process32First(snapshot_handle, &mut process_entry) == FALSE {
            // error!("Couldn't get first process.");
            CloseHandle(snapshot_handle);
            return;
        }

        loop {
            if process_entry.th32ParentProcessID == target_pid {
                pid_list.push(process_entry.th32ProcessID);
            }

            if Process32Next(snapshot_handle, &mut process_entry) == FALSE {
                break;
            }
        }

        CloseHandle(snapshot_handle);
    }
}

/// Get all thread id of a specific process
fn get_threads(target_pid: u32) -> Vec<u32> {
    let mut threads = Vec::new();

    unsafe {
        // Take a snapshot of all threads in the system.
        // While enumerating the set of threads, new threads can be created and destroyed.
        let snapshot_handle = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if snapshot_handle == INVALID_HANDLE_VALUE {
            // error!("Failed to get process {target_pid} snapShot");
            return threads;
        }

        // Walk the list of threads.
        let mut thread_entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            ..Default::default()
        };
        if Thread32First(snapshot_handle, &mut thread_entry) == FALSE {
            // error!("Couldn't get first thread.");
            CloseHandle(snapshot_handle);
            return threads;
        }

        loop {
            if thread_entry.th32OwnerProcessID == target_pid {
                threads.push(thread_entry.th32ThreadID);
            }

            if Thread32Next(snapshot_handle, &mut thread_entry) == FALSE {
                break;
            }
        }

        CloseHandle(snapshot_handle);
    }

    threads
}

/// Suspend a thread
/// Each thread has a suspend count (with a maximum value of `MAXIMUM_SUSPEND_COUNT`).
/// If the suspend count is greater than zero, the thread is suspended; otherwise, the thread is not suspended and is eligible for execution.
/// Calling `SuspendThread` causes the target thread's suspend count to be incremented.
/// Attempting to increment past the maximum suspend count causes an error without incrementing the count.
/// [SuspendThread](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread)
fn suspend_thread(tid: u32) {
    unsafe {
        // Attempt to convert the thread ID into a handle
        let thread_handle = OpenThread(THREAD_SUSPEND_RESUME, FALSE, tid);
        if thread_handle != NULL {
            // If SuspendThread fails, the return value is (DWORD) -1
            if u32::max_value() == SuspendThread(thread_handle) {
                let err_code = GetLastError();
                // warn!("Failed to suspend thread {tid} with error code {err_code}");
            }
        }

        CloseHandle(thread_handle);
    }
}

/// Resume a thread
/// ResumeThread checks the suspend count of the subject thread.
/// If the suspend count is zero, the thread is not currently suspended. Otherwise, the subject thread's suspend count is decremented.
/// If the resulting value is zero, then the execution of the subject thread is resumed.
/// [ResumeThread](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread)
fn resume_thread(tid: u32) {
    unsafe {
        // Attempt to convert the thread ID into a handle
        let thread_handle = OpenThread(THREAD_SUSPEND_RESUME, FALSE, tid);
        if thread_handle != NULL {
            // If ResumeThread fails, the return value is (DWORD) -1
            if u32::max_value() == ResumeThread(thread_handle) {
                let err_code = GetLastError();
                // warn!("Failed to resume thread {tid} with error code {err_code}");
            }
        }

        CloseHandle(thread_handle);
    }
}

/// Assert that certain process id no longer exists
pub fn process_exists(pid: u32) -> bool {
    unsafe {
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        let mut process_entry = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        loop {
            if process_entry.th32ProcessID == pid {
                CloseHandle(handle);
                return true;
            }

            if Process32Next(handle, &mut process_entry) == FALSE {
                break;
            }
        }

        CloseHandle(handle);
    }

    false
}
