use pos2pot::{get_potcar_list, write_potcar_manually, write_recommended_potcar, Config};
use std::{env, fmt, process};
use vasp_parser::poscar::read_elems;

use inquire::Select;

enum RunMode {
    Prompt,
    Express,
}

enum WriteMode {
    Recommended,
    Manual,
    Exit,
}

impl fmt::Display for WriteMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            WriteMode::Recommended => "Recommended",
            WriteMode::Manual => "Manual",
            WriteMode::Exit => "Exit",
        };
        write!(f, "{}", printable)
    }
}

fn main() {
    let args = {
        let arglist: Vec<String> = env::args().collect();
        parse_args(arglist)
    };
    let config_path = home::home_dir().unwrap().join(".config/pos2pot");
    let config = Config::read(config_path.join("config.json"));

    let potcar_table = get_potcar_list(config_path.join("potcar.json"));

    // Read elements in POSCAR
    let elems = read_elems("POSCAR").expect("Error when reading POSCAR");
    println!("Found {} in POSCAR", elems.join(", "));

    match args {
        RunMode::Prompt => {
            let modelist: Vec<WriteMode> =
                vec![WriteMode::Recommended, WriteMode::Manual, WriteMode::Exit];
            let mode = Select::new("Select mode: ", modelist).prompt();
            match mode {
                Ok(mode) => match mode {
                    WriteMode::Recommended => {
                        println!("Using recommended POTCAR configurations...");
                        write_recommended_potcar(&elems, &potcar_table, &config.potcar_path);
                    }
                    WriteMode::Manual => {
                        println!("Manual mode");
                        write_potcar_manually(&elems, &potcar_table, &config.potcar_path);
                    }
                    WriteMode::Exit => {
                        println!("Exit pos2pot");
                        process::exit(0);
                    }
                },
                Err(_) => println!("Error"),
            }
        }
        RunMode::Express => {
            println!("Using reccomeended POTCAR configurations...");
            write_recommended_potcar(&elems, &potcar_table, &config.potcar_path);
        }
    }
}

fn parse_args(args: Vec<String>) -> RunMode {
    if args.len() > 2 {
        eprintln!("Error: Too many arguments. Available arguments: no arguments or 'express'");
        std::process::exit(1);
    } else if args.len() == 1 {
        RunMode::Prompt
    } else if (args.len() == 2) && (args[1] == *"express") {
        RunMode::Express
    } else {
        eprintln!("Error: Unknown argument. Available arguments: no arguments or 'express'");
        std::process::exit(1);
    }
}
