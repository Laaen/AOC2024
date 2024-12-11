use std::{collections::HashMap, fs};
use std::time;

fn transform(nb: u64) -> Vec<u64>{
    if nb == 0{
        return vec![1];
    } else if nb.to_string().len() % 2 == 0{
        let digits = nb.to_string();
        return vec![digits[0..(digits.len()/2)].parse().unwrap(), digits[(digits.len()/2)..].parse().unwrap()];
    } else{
        return vec![nb*2024];
    }
}

fn main() {

    let start = time::Instant::now();

    let data = fs::read_to_string("input").unwrap();
    let stones_list: Vec<u64> = data.split(" ").map(|x| x.parse().unwrap()).collect();

    let mut stones: HashMap<u64, u64> = HashMap::new();
    for s in stones_list{
        stones.entry(s).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut cache: HashMap<u64, Vec<u64>> = HashMap::new();

    for i in 0..75{
        let cpy = stones.clone();
        stones.clear();
        for (k, v) in cpy.iter(){
            let res = cache.entry(*k).or_insert(transform(*k));
            for elt in res{
                let val = stones.get(elt).unwrap_or(&0).clone();
                stones.insert(*elt, val + v);
            }
        }
    }

    println!("{:?}", stones.values().map(|x| *x as u64).sum::<u64>());
    println!("{:?}", time::Instant::now() - start);
}
