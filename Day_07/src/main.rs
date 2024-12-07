use std::{collections::HashSet, fs};

fn get_possibilities(previous : &Vec<i64>, element : i64) -> Vec<i64>{
    let mut res = vec![];
    for p in previous{
        res.push(p + element);
        res.push(p * element);
    }
    return res;
}

fn get_possibilities_bis(previous : &Vec<i64>, element : i64) -> Vec<i64>{
    let mut res = vec![];

    for p in previous{
        res.push(p + element);
        res.push(p * element);
        res.push(concat(*p, element));
    }
    return res;
}

fn concat(a: i64, b : i64) -> i64{
    ((a as f64) * 10_f64.powf((b as f64).log10().floor() + 1.0) + b as f64) as i64
}

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let equations_list : Vec<Vec<i64>> = data.split("\r\n").map(|line| line.split(" ").map(|elt| elt.strip_suffix(":").unwrap_or(elt).parse().unwrap()).collect()).collect();
    
    // Part 1

    let mut result_1 : HashSet<i64> = HashSet::new();

    for elt in equations_list.iter(){
        let mut res = vec![vec![elt[1] + elt[2], elt[1] * elt[2]]];
        for i in elt[3..].iter(){
            res.push(get_possibilities(&res.last().unwrap(), *i));
        }
        if res.last().unwrap().contains(&elt[0]){
            result_1.insert(elt[0]);
        }
    }

    println!("{:?}", result_1.iter().sum::<i64>());

    // Part 2
    let mut result_2 : HashSet<i64> = HashSet::new();

    for elt in equations_list{
        let mut res = vec![vec![elt[1] + elt[2], elt[1] * elt[2], concat(elt[1], elt[2])]];
        for i in elt[3..].iter(){
            res.push(get_possibilities_bis(&res.last().unwrap(), *i));
        }
        if res.last().unwrap().contains(&elt[0]){
            result_2.insert(elt[0]);
        }
    }

    println!("{:?}", result_2.iter().sum::<i64>());

}


