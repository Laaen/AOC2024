use std::{collections::HashMap, fs};
use regex::Regex;

fn main() {

    let mut wires: HashMap<&str, i32> = HashMap::new();

    let data = fs::read_to_string("input").unwrap();
    let tmp = data.split("\n\n").collect::<Vec<&str>>();
    let starting_input: Vec<&str> = tmp[0].split("\n").collect();

    // Setup the wires
    for elt in starting_input.iter(){
        let tmp: Vec<&str> = elt.split(": ").collect();
        wires.insert(tmp[0], tmp[1].parse().unwrap());
    }

    // Go through the gates !
    let re = Regex::new(r"([a-z0-9]{3}) (XOR|AND|OR) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();

    // Redo until all wires are done
    while wires.keys().len() != data.split("\n").collect::<Vec<&str>>().len() - 1{
        for (_, [src1, op, src2, res]) in re.captures_iter(tmp[1]).map(|c| c.extract()){
            // Don't redo if result already known
            if !wires.contains_key(res){
                if wires.contains_key(src1) && wires.contains_key(src2){
                    let result = match op {
                        "OR" => wires[src1] | wires[src2],
                        "AND" => wires[src1] & wires[src2],
                        "XOR" => wires[src1] ^ wires[src2],
                        _ => panic!("Unknown operation")
                    };
                    wires.insert(res, result);
                }
            }
        }
    }

    let mut res: Vec<(&str, i32)> = vec![];
    // Get zs
    for w in wires.keys().filter(|k| k.starts_with("z")){
        res.push((w, wires[w]));
    }

    res.sort_by(|a, b| a.0.cmp(b.0));
    let p_1: u64 = res.iter().enumerate().map(|(idx, val)| (val.1 as u64) * 2_u64.pow(idx as u32)).sum();

    println!("{:?}", p_1);

}
