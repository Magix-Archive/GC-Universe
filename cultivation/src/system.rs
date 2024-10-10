use std::env::{args, current_dir, set_current_dir, current_exe};
use std::process::{exit, Command};
use sysinfo::System;
use tokio::select;
use tokio::time::{Duration, sleep};
use crate::structs::State;

/// Runs a system command in a new thread.
/// program: The program to run.
/// args: The arguments to pass to the program.
/// relative: Whether to run the command in the program's directory.
pub fn run_command(program: &str, args: Vec<&str>, relative: Option<bool>) {
    let program = program.to_string();
    let args = args.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    std::thread::spawn(move || {
        // Fetch the current working directory.
        let cwd = current_dir().unwrap();

        if relative.unwrap_or(false) {
            // Move to the program's directory
            let mut path_buf = std::path::PathBuf::from(&program);
            path_buf.pop();

            set_current_dir(&path_buf).unwrap();
        }

        // Run the command.
        let mut command = Command::new(&program);
        command.args(&args);
        command.spawn().unwrap();

        // Restore the original working directory.
        set_current_dir(cwd).unwrap();
    });
}

#[cfg(windows)]
pub fn is_elevated() -> bool {
    is_elevated::is_elevated()
}

#[cfg(unix)]
pub fn is_elevated() -> bool {
    false
}

// Check if we are running as administrator.
pub fn elevate() {
    // If we specify that we don't require admin, we can skip this check.
    // We can also skip if we are already running as admin.
    if is_elevated() || !State::instance().require_admin {
        return;
    }

    // We can safely ignore if we are on a non-Windows platform.
    if !cfg!(target_os = "windows") {
        println!("You need to run this program as sudo!");
        exit(1);
    }

    use std::ptr;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::shellapi::ShellExecuteW;
    use winapi::um::winuser::SW_SHOW;

    let exe_path = current_exe().expect("Failed to get current executable path");
    let args: Vec<String> = args().skip(1)
        .collect(); // Skip the first argument, which is the path to the exe
    let args_string = args.join(" ");

    let operation = OsString::from("runas")
        .encode_wide().chain(Some(0)).collect::<Vec<u16>>();
    let file = OsString::from(exe_path)
        .encode_wide().chain(Some(0)).collect::<Vec<u16>>();
    let parameters = OsString::from(args_string)
        .encode_wide().chain(Some(0)).collect::<Vec<u16>>();

    unsafe {
        ShellExecuteW(
            ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            parameters.as_ptr(),
            ptr::null(),
            SW_SHOW,
        );
    }

    exit(0);
}

/// Waits for a process to close or for a CTRL + C.
/// process_name: The name of the process to wait for.
pub async fn wait_for_action(process_name: String) {
    select! {
        _ = wait_for_close(process_name) => {},
        _ = tokio::signal::ctrl_c() => {}
    }
}

/// Waits for the process to close.
/// process_name: The name of the process to wait for.
pub async fn wait_for_close(process_name: String) {
    let mut system = System::new_all();
    system.refresh_all();

    let mut seen_once = false;
    loop {
        system.refresh_all();

        let mut processes = system.processes_by_exact_name(&process_name);
        let seen = processes.next().is_some();

        if seen && !seen_once {
            seen_once = true;
        } else if !seen && seen_once {
            break;
        } else {
            sleep(Duration::from_secs(2)).await;
        }
    }
}

/// Fetches the path where the executable is located.
pub fn install_location() -> String {
    let mut exe_path = current_exe().unwrap();

    #[cfg(windows)]
    {
        exe_path.pop();
        return exe_path.to_str().unwrap().to_string();
    }

    #[cfg(target_os = "linux")]
    {
        let bin_name = exe_path.file_name().unwrap().to_str().unwrap().to_string();
        exe_path.pop();
        if exe_path.starts_with("/usr/bin") {
            let mut path = PathBuf::from("/usr/lib");
            path.push(bin_name);
            path
        } else {
            exe_path
        }
            .to_str()
            .unwrap()
            .to_string()
    }
}
