use std::fs;

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let stones: Vec<u64> = data.split(" ").map(|x| x.parse().unwrap()).collect();

    let mut result: Vec<u64> = Vec::new();
    let mut prev: Vec<u64> = stones.clone();

    for i in 0..75{
        println!("{i}");
        result.clear();
        for s in &prev{
            if *s == 0{
                result.push(1);
            } else if s.to_string().len() % 2 == 0{
                let digits = s.to_string();
                result.push(digits[0..(digits.len()/2)].parse().unwrap());
                result.push(digits[(digits.len()/2)..].parse().unwrap());
    
            } else{
                result.push(s * 2024);
            }
        }
        prev = result.clone();
        
    }

    println!("{:?}", result);

}
