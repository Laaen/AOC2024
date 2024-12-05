use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

// Sort_by

fn compare(a : i32, b : i32) -> Ordering{
    // Check if a in hash, if it is return less for each value
    return Ordering::Less;
}

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let tmp: Vec<_> = data.split("\n\n").collect();

    let mut rules : HashMap<i32, Vec<i32>> = HashMap::new();
    for elt in tmp[0].split("\n"){
        let splitted : Vec<i32> = elt.split("|").map(|x| x.parse().unwrap()).collect();
        rules.entry(splitted[0]).and_modify(|x| x.push(splitted[1])).or_insert(vec![splitted[1]]);
    }

    let data_to_check : Vec<Vec<i32>> = tmp[1].split("\n")
    .map(|elt| elt.split(",").map(|x| x.parse().unwrap()).collect()).collect();


    println!("{:?}", rules);

}
