// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn default_colors() -> serde_json::Value {
    let input: serde_json::Value = serde_yaml::from_str(&"---
name: 'Aci'

color_01: '#363636'    # Black (Host)
color_02: '#FF0883'    # Red (Syntax string)
color_03: '#83FF08'    # Green (Command)
color_04: '#FF8308'    # Yellow (Command second)
color_05: '#0883FF'    # Blue (Path)
color_06: '#8308FF'    # Magenta (Syntax var)
color_07: '#08FF83'    # Cyan (Prompt)
color_08: '#B6B6B6'    # White

color_09: '#424242'    # Bright Black
color_10: '#FF1E8E'    # Bright Red (Command error)
color_11: '#8EFF1E'    # Bright Green (Exec)
color_12: '#FF8E1E'    # Bright Yellow
color_13: '#1E8EFF'    # Bright Blue (Folder)
color_14: '#8E1EFF'    # Bright Magenta
color_15: '#1EFF8E'    # Bright Cyan
color_16: '#C2C2C2'    # Bright White

background: '#0D1926'  # Background
foreground: '#B4E1FD'  # Foreground (Text)

cursor: '#B4E1FD'      # Cursor
    ").unwrap();

    input
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            default_colors,
            network::get_networks,
            network::connect_network,
            network::connect_network_password,
            network::is_wifi_on,
            network::enable_wifi,
            network::disable_wifi,
            network::is_existing_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
