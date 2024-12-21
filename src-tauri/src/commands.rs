use regex::Regex;
use std::fs;
use std::process::Command;

#[tauri::command]
pub fn get_aws_profiles() -> Result<Vec<String>, String> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let config_path = home.join(".aws").join("config");

    let content =
        fs::read_to_string(config_path).map_err(|e| format!("Failed to read AWS config: {}", e))?;

    let profile_regex = Regex::new(r"\[profile\s+([^\]]+)\]|\[default\]").unwrap();

    let mut profiles = vec!["default".to_string()];
    for cap in profile_regex.captures_iter(&content) {
        if let Some(profile) = cap.get(1) {
            profiles.push(profile.as_str().to_string());
        }
    }

    Ok(profiles)
}

#[tauri::command]
pub async fn list_s3_buckets(profile: String) -> Result<String, String> {
    let mut cmd = Command::new("aws");

    if profile != "default" {
        cmd.args(["--profile", &profile]);
    }

    cmd.args(["s3", "ls"]);

    let output = cmd.output().map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
