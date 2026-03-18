pub fn is_debug_mode() -> bool {
    std::env::var("TADOKU_DEBUG")
        .map(|s| s == "1" || s.to_lowercase() == "true")
        .unwrap_or(false)
}
