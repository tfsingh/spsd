use std::error::Error;
use std::process::Command;
use std::str;

use crate::utils::{
    config::{get_app_name, write_config},
    types::Config,
};

pub fn modify_profile(api_key: &str, allocate_ip: bool) -> Result<String, Box<dyn Error>> {
    let output = Command::new("flyctl").arg("auth").arg("login").output()?;

    if !output.status.success() {
        return Err(format!(
            "Authentication failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

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

    let config = Config {
        fly_api_key: Some(api_key.to_owned()),
        fly_app_name: Some(app_name),
    };

    write_config(&config)?;

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
