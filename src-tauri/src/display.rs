use std::process::{Command};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct DisplaySize {
    name: String,
    width: u32,
    height: u32,
    refresh_rates: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Display {
    display_name: String,
    is_connected: bool,
    is_primary: bool,
    x: i32,
    y: i32,
    selected_size: usize,
    sizes: Vec<DisplaySize>,
}

#[derive(Serialize, Deserialize)]
pub struct ScreenConfiguration {
    name: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

// Display Configuraiton object passed from frontend
#[derive(Serialize, Deserialize)]
pub struct DisplayConfigurationFrontend {
    screens: Vec<ScreenConfiguration>
}

#[tauri::command]
pub fn get_display_configuration() -> Vec<Display> {
    let mut nmcli = Command::new("xrandr")
        .output()
        .expect("Unable to get xrandr output");

    let raw_string = String::from_utf8(nmcli.stdout).expect("Unable to parse output from powerprofilesctl");
    let lines = raw_string.split("\n").collect::<Vec<_>>();

    let mut displays: Vec<Display> = vec![];

    let mut display_name = "PLACE";
    let mut display_is_connected = true;
    let mut display_is_primary = false;
    let mut display_x = 0;
    let mut display_y = 0;

    let mut selected_index = 0;
    let mut display_sizes: Vec<DisplaySize> = vec![];
    
    for line in lines {
        if line.starts_with("Screen") || line.len() < 3 { continue };

        let trimmed = line.trim();


        if trimmed.chars().nth(0).unwrap().is_numeric() {
            // This line is a display size option
    
            if trimmed.contains("*") { selected_index = 0 }

            let cells: Vec<_> = trimmed.split_whitespace().collect();

            let size_name = cells[0];
            let clean_name = size_name.replace("i", "");
            let split_name: Vec<_> = clean_name.split("x").collect();

            let width = split_name[0].parse::<u32>().unwrap();
            let height = split_name[1].parse::<u32>().unwrap();

            let mut rates: Vec<f32> = vec![];
            for c in &cells[1..] {
                let clean_cell = c.replace("*+", "");
                rates.push(clean_cell.parse::<f32>().unwrap());
            }

            display_sizes.push(DisplaySize{
                name: size_name.to_string(),
                width: width,
                height: height,
                refresh_rates: rates,                
            });
        } else {
            if display_sizes.len() > 0 {
                // Push existing data to display and reset
                displays.push(Display{
                    display_name: display_name.to_string(),
                    is_connected: display_is_connected,
                    is_primary: display_is_primary,
                    x: display_x,
                    y: display_y,
                    selected_size: selected_index,
                    sizes: display_sizes,
                });


                display_name = "PLACE";
                selected_index = 0;
                display_is_primary = false;
                display_sizes = vec![];
            }

            if trimmed.contains("disconnected") { continue }

            let cells: Vec<_> = trimmed.split_whitespace().collect();
            display_name = cells[0];
            if cells[1] == "connected" {
                display_is_connected = true;
            }

            let mut index_dim = 2;
            if cells[2] == "primary" {
                display_is_primary = true;
                index_dim = 3
            } 

            let dim_split = cells[index_dim].split("+").collect::<Vec<_>>();

            println!("{:?}", dim_split);

            display_x = dim_split[1].parse::<i32>().unwrap();
            display_y = dim_split[2].parse::<i32>().unwrap();
        }
    }

    return displays;
}

#[tauri::command]
pub fn set_display_configuration(config: DisplayConfigurationFrontend) {
    let mut arg_string = "".to_string();
    for screen in config.screens {
        arg_string.push_str(&format!("--output {} --mode {}x{} --pos {}x{} --rotate normal ",
                screen.name,
                screen.width,
                screen.height,
                screen.x,
                screen.y));
    }

    println!("{}", arg_string);

    let args = arg_string.split_whitespace().collect::<Vec<_>>();

    let mut nmcli = Command::new("xrandr")
        .args(args)
        .output()
        .expect("Unable to get xrandr output");

    println!("{:?}", nmcli);
}
