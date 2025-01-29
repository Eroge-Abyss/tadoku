use std::path::PathBuf;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};
use url::Url;

/// Extracts an image filename from an image URL
pub fn extract_image(url: &str) -> String {
    // TODO: Add proper error handling
    let url = Url::parse(url).expect("Failed to parse URL");
    url.path_segments()
        .expect("Failed to get segments")
        .last()
        .expect("Failed to get filename")
        .to_string()
}

pub fn construct_image_path(base_path: &PathBuf, url: &str) -> PathBuf {
    base_path.join("images").join(extract_image(url))
}

/// Gets the playtime of the current game in seconds
pub fn get_playtime(pid: u32) -> Option<u64> {
    // Is this bad for performance? to create a system instance on each call
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    s.process(Pid::from(pid as usize))
        .map(|process| process.run_time())
}
