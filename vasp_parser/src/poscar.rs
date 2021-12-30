/// Parse POSCAR. Currently implemented feature is reading elems.
/// TODO: add Poscar struct
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
