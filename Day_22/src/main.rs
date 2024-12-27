use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Buyer{
    data: HashMap<[i32; 4], u64>
}

impl Buyer{
    fn get(&self, seq: &[i32; 4]) -> Option<&u64>{
        self.data.get(seq)
    }
}

fn first_step(nb: u64) -> u64{
    ((nb * 64) ^ nb) % 16777216  
}

fn second_step(nb: u64) -> u64{
    ((nb / 32) ^ nb) % 16777216
}

fn third_step(nb: u64) -> u64{
    ((nb * 2048) ^ nb) % 16777216
}

fn generate_secret_numbers(starting_nb: u64, gen_nb: i32) -> Vec<u64>{
    let mut res: Vec<u64> = vec![starting_nb];
    for i in 0..gen_nb{
        res.push(third_step(second_step(first_step(*res.last().unwrap()))));
    }
    return res;
}

fn generate_prices(starting_nb: u64, gen_nb: i32) -> Vec<u64>{
    generate_secret_numbers(starting_nb, gen_nb).iter()
    .map(|x| x.to_string().chars().last().unwrap().to_digit(10).unwrap() as u64)
    .collect::<Vec<u64>>()
}

fn create_buyer(starting_nb: u64, gen_nb: i32) -> Buyer{
    let prices = generate_prices(starting_nb, gen_nb);
    let differences: Vec<i32> = prices.windows(2)
    .map(|a| a[1] as i32 - a[0] as i32)
    .collect();

    let mut buyer_data: HashMap<[i32; 4], u64> = HashMap::new();
    for (sequence, price) in differences.windows(4).zip(&prices[4..]){
        if !buyer_data.contains_key(sequence){
            buyer_data.insert(sequence.try_into().unwrap(), *price);
        }
    }

    return Buyer{data: buyer_data};
}

fn main() {

    let data = fs::read_to_string("input").unwrap();
    let secret_numbers: Vec<u64> = data.split("\n")
    .map(|elt| elt.parse::<u64>().unwrap())
    .collect();

    let mut res = 0;
    for elt in secret_numbers.iter(){
        res += generate_secret_numbers(elt.clone(), 2000).last().unwrap();
    }

    println!("Part 1 : {:?}", res);

    let mut buyers: Vec<Buyer> = vec![];

    for elt in secret_numbers.iter(){
        buyers.push(create_buyer(elt.clone(), 2000));
    }

    // Bruteforce again
    let mut explored: Vec<[i32; 4]> = vec![];
    let mut current_max_bananas = 0;
    let mut current_slice: [i32; 4] = [0,0,0,0];

    for b in buyers.iter(){
        for s in b.data.keys(){
            if !explored.contains(s){
                let mut banana_acc = 0;
                for elt in buyers.iter(){
                    banana_acc += elt.get(s).unwrap_or(&0);
                }
                if banana_acc > current_max_bananas{
                    current_slice = *s;
                    current_max_bananas = banana_acc;
                }
                explored.push(*s);
            }
        }
    }

    println!("Part 2 : {:?}", current_max_bananas);
    //println!("{:?}", current_slice);
    
}
