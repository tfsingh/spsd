pub fn parse_cpu(count: &str) -> Result<u32, String> {
    let value: u32 = count.parse().map_err(|_| "Invalid number of CPUs")?;

    if value >= 1 && value <= 16 {
        Ok(value)
    } else {
        Err("Number of CPUs must be between 1 and 16".to_string())
    }
}

pub fn parse_image(image: &str) -> Result<String, String> {
    match image {
        "base" => Ok(String::from("registry-1.docker.io/library/ubuntu:latest")),
        "python" => Ok(String::from("registry-1.docker.io/library/python:latest")),
        "rust" => Ok(String::from("registry-1.docker.io/library/rust:latest")),
        "go" => Ok(String::from("registry-1.docker.io/library/golang:latest")),
        "node" => Ok(String::from("registry-1.docker.io/library/node:latest")),
        _ => Ok(image.to_string()),
    }
}

pub fn parse_memory(amount: &str) -> Result<u32, String> {
    let value: u32 = amount.parse().map_err(|_| "Invalid amount of memory")?;

    let rounded_value = (value + 128) / 256 * 256;

    if rounded_value >= 256 && rounded_value <= 32768 {
        Ok(rounded_value)
    } else {
        Err("Memory must be between 256 and 32768".to_string())
    }
}

pub fn parse_volume(size: &str) -> Result<u32, String> {
    let value: u32 = size.parse().map_err(|_| "Invalid volume size")?;

    if value >= 1 && value <= 50 {
        Ok(value)
    } else {
        Err("Size of volume must be between 1 and 500 gb".to_string())
    }
}

pub fn parse_port(port: &str) -> Result<u16, String> {
    let value: u16 = port.parse().map_err(|_| "Invalid port")?;

    if value >= 1024 {
        Ok(value)
    } else {
        Err("Port must be between 1024 and 65535".to_string())
    }
}

pub fn parse_region(region: &str) -> Result<String, String> {
    let allowed_regions = vec![
        "ams", "bom", "cdg", "dfw", "fra", "hkg", "iad", "lax", "lhr", "nrt", "ord", "scl", "sea",
        "sin", "sjc", "syd", "yyz",
    ];

    if allowed_regions.contains(&region) {
        Ok(region.to_string())
    } else {
        Err("Invalid region".to_string())
    }
}
