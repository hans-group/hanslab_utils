use crate::poscar::Poscar;
use ndarray::Array;
use ndarray::Array2;
use roxmltree::{Document, Node};
use std::fmt::Debug;
use std::str::FromStr;

pub struct Vasprun {
    pub poscar: Poscar,
    pub contcar: Poscar,
    pub final_energy: f64,
    pub calculations: Vec<CalcStep>,
}

#[allow(unused)]
impl<'a> Vasprun {
    pub fn from_file(filename: &'a str) -> Vasprun {
        let contents = std::fs::read_to_string(filename).unwrap();
        let vasprun_root = { roxmltree::Document::parse(&contents).unwrap() };
        let (poscar, contcar) = Vasprun::get_poscar_and_contcar(&vasprun_root);

        let calculations = {
            let calcs = get_calculation_nodes(&vasprun_root)
                .map(|node| CalcStep::from_node(&poscar.symbols, &node));
            calcs.collect::<Vec<CalcStep>>()
        };
        let final_energy = calculations.iter().last().unwrap().energy;

        Vasprun {
            poscar,
            contcar,
            calculations,
            final_energy,
        }
    }

    fn get_poscar_and_contcar(vasprun_root: &'a Document) -> (Poscar, Poscar) {
        let symbols = get_symbols(vasprun_root);
        let get_structure_node = |attr: &str| {
            vasprun_root
                .descendants()
                .filter(|n| n.attribute("name") == Some(attr))
                .nth(0)
                .unwrap()
        };
        let poscar = {
            let node = get_structure_node("initialpos");
            get_poscar_from_node(&symbols, &node)
        };
        let contcar = {
            let node = get_structure_node("finalpos");
            get_poscar_from_node(&symbols, &node)
        };

        (poscar, contcar)
    }
}

fn get_poscar_from_node<'a>(symbols: &Vec<String>, structure_node: &'a Node) -> Poscar {
    let cell = get_node_by_attr(&structure_node, "basis");
    let positions = get_node_by_attr(&structure_node, "positions");
    let selective_dynamics: Option<Array2<char>> = {
        let node = structure_node
            .descendants()
            .filter(|n| n.attribute("name") == Some("selective"))
            .nth(0);
        match node {
            Some(node) => Some(parse_varray(&node)),
            None => None,
        }
    };
    Poscar {
        cell: parse_varray(&cell),
        positions: parse_varray(&positions),
        symbols: symbols.to_vec(),
        selective_dynamics,
    }
}

#[derive(Debug)]
pub struct CalcStep {
    pub structure: Poscar,
    pub energy: f64,
    pub forces: Array2<f64>,
}

impl<'a> CalcStep {
    pub fn from_node(symbols: &Vec<String>, calc_node: &'a Node) -> CalcStep {
        let structure_node = calc_node
            .descendants()
            .filter(|n| n.has_tag_name("structure"))
            .nth(0)
            .unwrap();
        let structure = get_poscar_from_node(symbols, &structure_node);
        let energy = get_e0(calc_node);
        let forces = get_forces(calc_node);
        CalcStep {
            structure,
            energy,
            forces,
        }
    }
}

pub fn get_calculation_nodes<'a>(
    vasprun_root: &'a Document,
) -> Box<dyn Iterator<Item = Node<'a, 'a>> + 'a> {
    let calculations = vasprun_root
        .descendants()
        .filter(|n| n.has_tag_name("calculation"));
    let result = Box::new(calculations);
    result
}

fn get_e0<'a>(calc_node: &'a Node) -> f64 {
    let energy_node = calc_node
        .children()
        .filter(|n| n.has_tag_name("energy"))
        .nth(0)
        .expect("Step not finished");
    let energy_zero = energy_node.children().nth(1).unwrap();

    energy_zero
        .text()
        .unwrap()
        .trim()
        .parse()
        .expect("Error in parsing float")
}

fn get_forces<'a>(calc_node: &'a Node) -> Array2<f64> {
    let forces_node = calc_node
        .children()
        .filter(|n| n.has_tag_name("varray") && n.attribute("name") == Some("forces"))
        .nth(0)
        .unwrap();

    parse_varray(&forces_node)
}

fn get_symbols<'a>(vasprun_root: &'a Document) -> Vec<String> {
    let atominfo = vasprun_root
        .descendants()
        .filter(|n| n.attribute("name") == Some("atoms"))
        .nth(0)
        .unwrap();
    atominfo
        .descendants()
        .filter(|n| n.has_tag_name("rc"))
        .map(|n| n.first_child().unwrap().text().unwrap().trim().to_string())
        .collect()
}

fn parse_varray<'a, T>(n: &'a Node) -> Array2<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut arr: Vec<T> = vec![];
    let arr_elems = n.children().filter(|n| n.has_tag_name("v"));
    for v in arr_elems {
        let row: Vec<T> = v
            .text()
            .unwrap()
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        arr.extend(row);
    }
    Array::from_shape_vec((arr.len() / 3, 3), arr).expect("Error in parsing varray")
}

fn get_node_by_attr<'a>(node: &'a Node, attr: &str) -> Node<'a, 'a> {
    node.descendants()
        .filter(|n| n.attribute("name") == Some(attr))
        .nth(0)
        .unwrap()
}
