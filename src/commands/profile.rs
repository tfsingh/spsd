use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::str;

use crate::utils::constants::get_app_name;

pub fn modify_profile(api_key: &str, allocate_ip: bool) -> Result<String, Box<dyn Error>> {
    let output = Command::new("flyctl")
        .arg("auth")
        .arg("token")
        .arg("-t")
        .arg(api_key)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Authentication failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(".env")?;

    let result = get_app_name();
    let app_name = match result {
        Ok(app_name) => app_name,
        Err(_) => {
            let output = Command::new("flyctl")
                .arg("apps")
                .arg("create")
                .arg("--generate-name")
                .output()?;

            if !output.status.success() {
                return Err(format!(
                    "App creation failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
                .into());
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout
                .split('\n')
                .find(|line| line.contains("New app created:"))
                .and_then(|line| line.split_whitespace().last())
                .ok_or("App name not found in output")?
                .to_string()
        }
    };

    file.set_len(0)?;
    writeln!(file, "FLY_APP_NAME={}", app_name)?;
    writeln!(file, "FLY_API_KEY={}", api_key)?;

    if allocate_ip {
        let mut child = Command::new("flyctl")
            .arg("ip")
            .arg("allocate-v4")
            .spawn()?;
        let status = child.wait().unwrap();

        if !status.success() {
            return Err("IP allocation failed".into());
        }
    }

    Ok(String::from("Set app and API key"))
}
