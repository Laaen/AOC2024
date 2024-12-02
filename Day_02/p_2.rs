use std::fs;

fn check_line(line : &Vec<i32>) -> bool{

    let mut unordered_num = 0;
    // Number of numbers not in order
    for elt in line.windows(3){
        println!("{:?}", elt);
        if (elt[0] - elt[1]) * (elt[1] - elt[2]) < 0{
            unordered_num += 1;
        }
    }
    
    // Divise par 2, car chaque nombre non ordonné compte pour 2
    unordered_num /= 2;

    // Errors counter
    let mut errors = 0;

    for elt in line.windows(3){
        // If not sorted, add an error
        if !(elt.is_sorted_by(|a, b| a <= b) || elt.is_sorted_by(|a, b| b <= a)){
            errors += 1;
        }
        if (elt[0] - elt[1]).abs() > 3 || (elt[0] - elt[1]).abs() == 0{
                errors += 1;
            if (elt[0] - elt[2]).abs() > 3 || (elt[0] - elt[2]).abs() == 0{
                errors += 1;
            }
        }

    }

    println!("{:?}", line);
    println!("{:?}", errors);

    return errors <= 1;
}

fn main(){
    let input = fs::read_to_string("input").expect("Error");
    let data : Vec<Vec<i32>> = input.split("\n").map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect()).collect();

    let result = data.iter().map(|elt| check_line(elt)).filter(|elt| *elt == true).count();


    println!("{:?}", result);
}