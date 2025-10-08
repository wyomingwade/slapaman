pub fn memory_value_coerced(memory: Option<u32>) -> u32 {
    // leaving memory blank will default to 2048 MB
    match memory {
        Some(m) => m,
        None => 2048,
    }
}

pub fn parse_mem(s: &str) -> Result<u32, String> {
    let suffixed = |mult| s[..s.len() - 1].parse::<u32>().map(|v| v * mult);
    match s.to_ascii_lowercase().as_str() {
        v if v.ends_with('g') => Ok(suffixed(1024).unwrap()),
        v if v.ends_with('m') => Ok(suffixed(1).unwrap()),
        v => v.parse::<u32>().map_err(|e| e.to_string()),
    }
}
