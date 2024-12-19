use std::{collections::HashMap, fs};
use rayon::prelude::*;

fn solve_for_design(design: &str, possibilities: &Vec<&str>, cache: &mut HashMap<String, bool>, result: &mut i32) -> bool{
    
    let mut ok = false;

    println!("{design}");
    // Check if res in cache
    if let Some(res) = cache.get(design) {
        return *res;
    }

    if design == ""{
        //*result += 1;
        ok = true;
    }

    for p in possibilities{
        if let Some(v) = design.strip_suffix(p){
            ok = ok || solve_for_design(v, &possibilities, cache, result);
            cache.entry(design.to_string()).insert_entry(ok);

        }
        if let Some(v) = design.strip_prefix(p){
            ok = ok || solve_for_design(v, &possibilities, cache, result);
            cache.entry(design.to_string()).insert_entry(ok);
        }
    }

    return ok;
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let tmp: Vec<&str> = input.split("\n\n").collect();

    let mut possibilities: Vec<&str> = tmp[0].split(", ").collect();
    let mut designs: Vec<&str> = tmp[1].split("\n").collect();

    possibilities.sort_by(|a, b| a.len().cmp(&b.len()));

    let res: Vec<(i32)> = designs.par_iter()
    .map(|elt| {
        let mut counter = 0;
        let res = solve_for_design(elt, &possibilities, &mut HashMap::<String, bool>::new(), &mut counter);
        (res, counter)
    })
    .filter(|x| x.0 == true)
    .map(|x| x.1)
    .collect();

    println!("Partie 1 : {:?} Partie 2 : {:?}", res.iter().count(), res.iter().sum::<i32>());

}
