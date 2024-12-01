use std::fs;
use std::str;
use std::iter;

fn main(){
    let data : Vec<Vec<i32>> = fs::read_to_string("input")
    .expect("Error")
    .split("\r\n")
    .map(|elt| elt.split("   ").map(|x| x.parse().unwrap()).collect())
    .collect();

    let mut list_a : Vec<i32> = data.iter().map(|elt| elt[0]).collect();
    let mut list_b : Vec<i32> = data.iter().map(|elt| elt[1]).collect();

    let result : i32 = list_a.iter().fold(0, |acc, x| acc + (x * list_b.iter().filter(|y| x == *y).count() as i32));

    println!("{:?}", result);
}