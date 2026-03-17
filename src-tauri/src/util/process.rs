use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

/// Gets the PID of a saved game's process file path
pub fn get_pid_from_process_path(process_file_path: &str) -> Option<Pid> {
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    for process in s.processes().values() {
        if let Some(exe) = process.exe() {
            if exe.to_str()? == process_file_path {
                return Some(process.pid());
            }

            #[cfg(not(windows))]
            {
                let normalized_path = process
                    .cmd()
                    .iter()
                    .filter_map(|s| s.to_str())
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .replace("\\", "/");

                if normalized_path.contains(process_file_path) {
                    return Some(process.pid());
                }
            }
        }
    }

    None
}
