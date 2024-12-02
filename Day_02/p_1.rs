use std::fs;

fn check_line(line : &Vec<i32>) -> bool{
    
    let sorted = line.is_sorted_by(|a, b| a <= b) || line.is_sorted_by(|a, b| b <= a);
    let steps_ok = line.windows(2).map(|elt| (elt[0] - elt[1]).abs()).all(|elt| elt <= 3 && elt != 0);

    return sorted && steps_ok;
}

fn main(){
    let input = fs::read_to_string("input").expect("Error");
    let data : Vec<Vec<i32>> = input.split("\n").map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect()).collect();

    let result = data.iter().map(|elt| check_line(elt)).filter(|elt| *elt == true).count() as i32;

    println!("{result}");
}