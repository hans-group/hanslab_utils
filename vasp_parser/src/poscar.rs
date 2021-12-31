use ndarray::Array2;

/// Parse POSCAR. Currently implemented feature is reading elems.
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Poscar {
    pub cell: Array2<f64>,
    pub positions: Array2<f64>,
    pub elements: Vec<String>,
    pub symbols: Vec<String>,
    pub selective_dynamics: Option<Array2<bool>>,
}

impl Poscar {
    pub fn from_file(filename: &str) -> Poscar {
        let f = File::open(filename).expect("Error in file opening");
        let file = BufReader::new(&f);
        let lines = file
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();
        let scaling_factor: f64 = lines[1].trim().parse().unwrap();

        let skip_to_pos: usize = {
            if lines[7].starts_with('S') {
                9
            } else {
                8
            }
        };

        // Read elements and symbols
        let elements = lines[5].split_whitespace().collect::<Vec<&str>>();
        let num_elems = lines[6]
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().expect("Error in parsing number"));
        let mut symbols: Vec<String> = vec![];
        for (&elem, num) in elements.iter().zip(num_elems) {
            (0..num).for_each(|_| symbols.push(elem.to_string()));
        }
        let num_atoms = symbols.len();

        let cell = Poscar::get_cell(&lines, scaling_factor);
        let positions = Poscar::get_positions(&lines, num_atoms, skip_to_pos);
        let selective_dynamics = Poscar::get_selective_dynamics(&lines, num_atoms, skip_to_pos);

        Poscar {
            cell,
            positions,
            elements: elements.iter().map(|&x| x.to_string()).collect(),
            symbols,
            selective_dynamics,
        }
    }

    fn get_cell(lines: &[String], scaling_factor: f64) -> Array2<f64> {
        let mut _cell = vec![];
        for l in lines.iter().take(5).skip(2) {
            let row = l
                .split_whitespace()
                .map(|x| x.trim().parse::<f64>().expect("Error in parsing number"));
            row.for_each(|x| _cell.push(scaling_factor * x));
        }
        Array2::from_shape_vec((3, 3), _cell).unwrap()
    }

    fn get_positions(lines: &[String], num_atoms: usize, skip_num: usize) -> Array2<f64> {
        let mut _positions = vec![];
        for l in lines.iter().skip(skip_num) {
            let row = l
                .split_whitespace()
                .take(3)
                .map(|x| x.parse::<f64>().expect("Error in parsing number"));
            row.for_each(|x| _positions.push(x));
        }
        Array2::from_shape_vec((num_atoms, 3), _positions).unwrap()
    }

    fn get_selective_dynamics(
        lines: &[String],
        num_atoms: usize,
        skip_num: usize,
    ) -> Option<Array2<bool>> {
        if skip_num == 9 {
            let mut _positions = vec![];
            for l in lines.iter().skip(skip_num) {
                let row = l.split_whitespace().skip(3);

                row.for_each(|x| {
                    if x == "T" {
                        _positions.push(true)
                    } else {
                        _positions.push(false)
                    }
                });
            }
            Some(Array2::from_shape_vec((num_atoms, 3), _positions).unwrap())
        } else {
            None
        }
    }
}

pub fn read_elems(poscar: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let f = File::open(poscar).expect("Error in file opening");
    let file = BufReader::new(&f);

    let mut elems: Vec<String> = vec![];
    for (i, line) in file.lines().enumerate() {
        let l: String = line?;
        if i == 5 {
            elems = l.split_whitespace().map(str::to_string).collect();
            break;
        }
    }
    Ok(elems)
}

#[cfg(test)]
mod tests {
    use crate::poscar::Poscar;
    #[test]
    fn test_read_poscar() {
        let pwd = std::process::Command::new("sh")
            .args(["-c", "pwd"])
            .output()
            .unwrap();
        println!("{:?}", pwd);
        let poscar = Poscar::from_file("../tests/selective_dyn.vasp");
        println!("with_selective_dyn: {:#?}", poscar);

        let poscar = Poscar::from_file("../tests/no_selective_dyn.vasp");
        println!("with_selective_dyn: {:#?}", poscar);
    }
}
