use url::Url;

pub fn extract_image(url: &str) -> String {
    // TODO: Add proper error handling
    let url = Url::parse(url).expect("Failed to parse URL");
    url.path_segments()
        .expect("Failed to get segments")
        .last()
        .expect("Failed to get filename")
        .to_string()
}
