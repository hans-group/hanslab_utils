use inquire::formatter::OptionFormatter;
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub potcar_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct PotcarData {
    pub element: String,
    pub potcar_name: String,
    pub enmax: i32,
    pub recommended: bool,
}

impl Config {
    pub fn read(filepath: PathBuf) -> Config {
        let config_file = fs::read_to_string(filepath);
        match config_file {
            Ok(c) => {
                let config_data: Config = serde_json::from_str(&c).expect("Parse error");
                config_data
            }
            Err(msg) => {
                panic!("{}", msg);
            }
        }
    }
}

pub fn get_potcar_list(filepath: PathBuf) -> Vec<PotcarData> {
    let config_file = fs::read_to_string(filepath);
    match config_file {
        Ok(c) => {
            let potcar_list: Vec<PotcarData> = serde_json::from_str(&c).expect("Parse error");
            potcar_list
        }
        Err(msg) => {
            panic!("{}", msg);
        }
    }
}

fn get_recommended_potcar(elem: &str, potcar_data: &Vec<PotcarData>, potcar_path: &str) -> String {
    let mut filepath: String = String::new();
    for potcar in potcar_data {
        if (potcar.element == elem) && (potcar.recommended) {
            filepath = format!("{}/{}/POTCAR", potcar_path, potcar.potcar_name).to_string();
        }
    }
    filepath
}

pub fn write_recommended_potcar(
    elems: &Vec<String>,
    potcar_data: &Vec<PotcarData>,
    potcar_path: &str,
) {
    let mut file = File::create("POTCAR").expect("Creation failed");
    for elem in elems {
        let recommended = get_recommended_potcar(elem, potcar_data, potcar_path);
        let potcar_contents = fs::read_to_string(recommended).expect("Error in reading POTCAR");
        file.write_all(potcar_contents.as_bytes())
            .expect("Failed to write");
    }
    println!("Wrote POTCAR sucessfully");
}

pub fn write_potcar_manually(
    elems: &Vec<String>,
    potcar_data: &Vec<PotcarData>,
    potcar_path: &str,
) {
    let formatter: OptionFormatter<String> = &|i| {
        let words: Vec<String> = i
            .to_string()
            .split_whitespace()
            .map(str::to_string)
            .collect();
        format!("{} selected", words[0])
    };
    let mut file = File::create("POTCAR").expect("Creation failed");
    for elem in elems {
        let mut potcar_list: Vec<String> = vec![];
        let mut potcar_info_list: Vec<String> = vec![];
        for potcar in potcar_data {
            if &potcar.element == elem {
                potcar_list.push(potcar.potcar_name.clone());
                let potcar_info: String = format!(
                    "{:<10}{:<15}{:<6}",
                    potcar.potcar_name, potcar.enmax, potcar.recommended
                );
                potcar_info_list.push(potcar_info);
            }
        }
        let potcar_info_table = potcar_info_list
            .iter()
            .zip(potcar_list.iter())
            .collect::<HashMap<_, _>>();

        let potcar_name = Select::new(
            format!(
                "Choose POTCAR for {}\n  {:<10}{:<15}{}",
                elem, "Name", "Cutoff (eV)", "Recommended"
            )
            .as_str(),
            potcar_info_list.clone(),
        )
        .with_formatter(formatter)
        .prompt()
        .unwrap();

        let selected_potcar_name = potcar_info_table.get(&potcar_name).unwrap();
        let filepath = format!("{}/{}/POTCAR", potcar_path, selected_potcar_name).to_string();

        let potcar_contents = fs::read_to_string(filepath).expect("Error in reading POTCAR");
        file.write_all(potcar_contents.as_bytes())
            .expect("Failed to write");
    }
    println!("Wrote POTCAR sucessfully");
}
