use inquire::formatter::OptionFormatter;
use inquire::Select;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::Write;

const POTCAR_FILEPATH_PREFIX: &str = "/TGM/Apps/VASP/POTCAR/2.POTPAW.PBE.54.RECOMMEND";

pub struct PotcarData {
    pub element: String,
    pub name: String,
    pub encut: i32,
    pub recommended: bool,
}

fn read_csv(file_path: &OsString) -> Result<csv::Reader<std::fs::File>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let rdr = csv::Reader::from_reader(file);
    Ok(rdr)
}
pub fn parse_potcar_table(filename: &OsString) -> Vec<PotcarData> {
    let mut potcar_table: Vec<PotcarData> = vec![];
    let mut rdr = read_csv(filename).expect("Error in reading file");
    for record in rdr.records() {
        let record = record.unwrap();
        let data = PotcarData {
            element: String::from(&record[0]),
            name: String::from(&record[1]),
            encut: record[2].parse().unwrap(),
            recommended: String::from(&record[3])
                .to_lowercase()
                .parse()
                .expect("Invalid string for boolean type"),
        };
        potcar_table.push(data);
    }

    potcar_table
}

fn get_recommended_potcar(elem: &str, potcar_table: &Vec<PotcarData>) -> String {
    let mut filepath: String = String::new();
    for potcar in potcar_table {
        if (potcar.element == elem) && (potcar.recommended) {
            filepath = format!("{}/{}/POTCAR", POTCAR_FILEPATH_PREFIX, potcar.name).to_string();
        }
    }
    filepath
}

pub fn write_recommended_potcar(elems: &Vec<String>, potcar_table: &Vec<PotcarData>) {
    let mut file = File::create("POTCAR").expect("Creation failed");
    for elem in elems {
        let recommended = get_recommended_potcar(elem, potcar_table);
        let potcar_contents = fs::read_to_string(recommended).expect("Error in reading POTCAR");
        file.write_all(potcar_contents.as_bytes())
            .expect("Failed to write");
    }
    println!("Wrote POTCAR sucessfully");
}

pub fn write_potcar_manually(elems: &Vec<String>, potcar_table: &Vec<PotcarData>) {
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
        for potcar in potcar_table {
            if &potcar.element == elem {
                potcar_list.push(potcar.name.clone());
                let potcar_info: String = format!(
                    "{:<10}{:<15}{:<6}",
                    potcar.name, potcar.encut, potcar.recommended
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
        let filepath =
            format!("{}/{}/POTCAR", POTCAR_FILEPATH_PREFIX, selected_potcar_name).to_string();

        let potcar_contents = fs::read_to_string(filepath).expect("Error in reading POTCAR");
        file.write_all(potcar_contents.as_bytes())
            .expect("Failed to write");
    }
    println!("Wrote POTCAR sucessfully");
}
