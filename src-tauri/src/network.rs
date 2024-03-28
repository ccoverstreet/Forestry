
use serde::{Serialize, Deserialize};
use std::process::{Command};

#[derive(Serialize, Deserialize)]
pub struct Network {
    connected: bool,
    bssid: String,
    ssid: String,
    mode: String,
    chan: u32,
    rate: i32, // Rate in Mbit/s 
    signal: u16,
    security: String,
}

#[tauri::command]
pub async fn get_networks() -> Vec<Network> {
    //let mut nmcli = Command::new("nmcli")
    //    .arg("dev")
    //    .arg("cli")
    //    .output()
    //    .expect("Unable to get available networks");

    let mut nmcli = Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .output()
        .expect("Unable to get available networks");

    let raw_string = String::from_utf8(nmcli.stdout).expect("");
    let lines: Vec<_>= raw_string
        .split("\n").collect::<Vec<_>>();



    let mut networks: Vec<Network> = vec![];
    for (i, line) in lines.iter().enumerate() {
        if i == 0 || line.len() < 15 { continue; }
        let comp = line.split_whitespace().collect::<Vec<_>>();
        println!("{} {:?}", line, comp);

        // This logic should be cleaned up in the future
        let mut index = 0;
        let mut connected = false;
        if comp[0] == "*" { 
            index += 1; 
            connected = true;
        }

        let bssid = comp[index];
        index += 1;

        let mut ssid = "".to_string();
        while index < comp.len() {
            if comp[index] == "Infra" 
                || comp[index] == "Ad-Hoc" { break }
            ssid.push_str(comp[index]);
            index += 1;
        }

        let mode = comp[index];
        index += 1;

        let chan: u32 = comp[index].parse().unwrap();
        index += 1;

        let rate_val: i32 = comp[index].parse().unwrap();
        index += 1;
        let _rate_unit = comp[index];
        index += 1;

        let signal: u16 = comp[index].parse().unwrap();
        index += 1;

        println!("{}", comp[index]);

        // Skip bars
        index += 1;


        let mut security = "".to_string();
        while index < comp.len() {
            security.push_str(comp[index]);
            index += 1;
        }


        networks.push(Network{
            connected: connected,
            bssid: bssid.to_string(),
            ssid: ssid.to_string(),
            mode: mode.to_string(),
            chan: chan,
            rate: rate_val,
            signal: signal,
            security: security.to_string(),
        });
    }

    println!("NMCLI output {:?} {:?}", nmcli.status, lines);

    return networks;
}


#[tauri::command]
pub fn connect_network(bssid: String) {
    let mut nmcli = Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .arg("connect")
        .arg(bssid)
        .output()
        .expect("Unable to connect");
}
