use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn compare(rules : &HashMap<i32, Vec<i32>>, a : i32, b : i32) -> Ordering{
    // Check if a in hash, if it is return less for each value
    if a == b{
        return Ordering::Equal
    }
    if rules.get(&a).unwrap_or(&vec![0]).contains(&b){
        return Ordering::Less;
    }
    return Ordering::Greater;
}

fn main() {

    let data = fs::read_to_string("input").unwrap();
    let tmp: Vec<_> = data.split("\r\n\r\n").collect();

    let mut rules : HashMap<i32, Vec<i32>> = HashMap::new();
    for elt in tmp[0].split("\r\n"){
        let splitted : Vec<i32> = elt.split("|").map(|x| x.parse().unwrap()).collect();
        rules.entry(splitted[0]).and_modify(|x| x.push(splitted[1])).or_insert(vec![splitted[1]]);
    }

    let mut data_to_check : Vec<Vec<i32>> = tmp[1].split("\r\n")
    .map(|elt| elt.split(",").map(|x| x.parse().unwrap()).collect()).collect();

    let mut res_1 = 0;
    let mut res_2 = 0;

    for elt in data_to_check.iter_mut(){

        if elt.is_sorted_by(|a, b| rules.get(a).unwrap_or(&vec![0]).contains(b)){
            res_1 += elt[elt.len()/2];
        }
        else{
            elt.sort_by(|a, b|compare(&rules, *a, *b));
            res_2 += elt[elt.len()/2];
        }
    }
    
    println!("{:?}", res_1);
    println!("{:?}", res_2);

}
