use std::process::{Command};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct PowerProfile {
    name: String,
    driver: String,
    is_active: bool,
}

#[tauri::command]
pub async fn get_power_profiles() -> Vec<PowerProfile> {
    let mut nmcli = Command::new("powerprofilesctl")
        .output()
        .expect("Unable to get power profiles");

    let raw_string = String::from_utf8(nmcli.stdout).expect("Unable to parse output from powerprofilesctl");
    let groups = raw_string.split("\n\n").collect::<Vec<_>>();

    let mut profiles: Vec<PowerProfile> = vec![];
    for group in groups {
        let lines: Vec<_> = group.split("\n").collect();

        let is_active = lines[0].starts_with("*");
        let split_1: Vec<_> = lines[0].split_whitespace().collect();

        let name = match split_1.last() {
            Some(name) => name.replace(":", ""),
            None => "Unnamed profile".to_string(),
        };

        let split_2: Vec<_> = lines[1].split_whitespace().collect();
        let driver = match split_2.last() {
            Some(d) => d.to_string(),
            None => "No driver specified".to_string(),
        };

        
        profiles.push(PowerProfile{
            name: name,
            driver: driver,
            is_active: is_active,
        });
    }

    return profiles;
}

#[tauri::command]
pub async fn set_power_profile(profile: PowerProfile) {
    let mut nmcli = Command::new("powerprofilesctl")
        .arg("set")
        .arg(profile.name)
        .output()
        .expect("Unable to set power profile");
}
