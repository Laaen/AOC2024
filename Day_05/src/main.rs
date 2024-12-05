use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::time;

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

fn custom_sort(rules : &HashMap<i32, Vec<i32>>, data : &mut Vec<i32>){
    data.sort_by(|a, b| compare(rules, *a, *b));

}

fn main() {

    let start = time::Instant::now();

    let data = fs::read_to_string("input").unwrap();
    let cleaned_data = data.replace("\r", "");
    let tmp: Vec<_> = cleaned_data.split("\n\n").collect();

    let mut rules : HashMap<i32, Vec<i32>> = HashMap::new();
    for elt in tmp[0].split("\n"){
        let splitted : Vec<i32> = elt.split("|").map(|x| x.parse().unwrap()).collect();
        rules.entry(splitted[0]).and_modify(|x| x.push(splitted[1])).or_insert(vec![splitted[1]]);
    }

    let mut data_to_check : Vec<Vec<i32>> = tmp[1].split("\n")
    .map(|elt| elt.split(",").map(|x| x.parse().unwrap()).collect()).collect();

    let mut part_1 : Vec<Vec<i32>> = Vec::new();
    let mut part_2 : Vec<Vec<i32>> = Vec::new();

    for elt in data_to_check{
        let mut copy = elt.clone();
        custom_sort(&rules, &mut copy);
        if copy == elt{
            part_1.push(elt);
        }
        else{
            part_2.push(copy);
        }
    }

    let res_1 = part_1.iter().map(|elt| elt[(elt.len()/2)]).fold(0, |res, a| res + a);
    let res_2 = part_2.iter().map(|elt| elt[(elt.len()/2)]).fold(0, |res, a| res + a);
    
    println!("{:?}", res_1);
    println!("{:?}", res_2);

    println!("Temps écoulé : {:?}", start.elapsed());

}
