use std::fs;
use std::time;


fn get_neighbour(data: &Vec<&str>, x : i32, y : i32) -> Vec<(char, i32, i32)>{

    let mut result : Vec<(char, i32, i32)> = Vec::new();
 
    for i in -1..2{
        result.push((data[x as usize].chars().nth((y + i) as usize).unwrap_or('L'), x, (y + i)));
        if x > 0{
            result.push((data[(x - 1) as usize].chars().nth((y + i) as usize).unwrap_or('L'), (x - 1),(y + i)));
        }
        if x < (data.len() - 1) as i32{
            result.push((data[(x + 1) as usize].chars().nth((y + i) as usize).unwrap_or('L'), (x + 1),(y + i)));
        }
    }

    return result;
}

/// Takes the next 4 letters if possible, and checks if XMAS is in
fn explore_direction(data: &Vec<&str>, current_x : i32, current_y : i32, offset_x : i32, offset_y : i32) -> bool{
    
    if (current_x + 3*offset_x) < 0 || (current_x + 3*offset_x) >= data.len() as i32{
        return false;
    }

    let mut result : Vec<char> = Vec::new();

    for i in 0..4{
        result.push(data[(current_x + i * offset_x) as usize].chars().nth((current_y + i * offset_y) as usize).unwrap_or('L'));
    }
    
    return result == vec!['X', 'M', 'A', 'S'];
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let lines: Vec<_> = content.split("\r\n").collect();

    let mut counter_1 = 0;

    // Part 1
    for (x, _) in lines.iter().enumerate(){
        for (y, _) in lines[x].char_indices(){
            if lines[x].chars().nth(y).unwrap() == 'X'{
                for (l, found_x, found_y) in get_neighbour(&lines, x as i32, y as i32){
                    if l == 'M'{
                        if explore_direction(&lines, x as i32, y as i32,found_x - x as i32, found_y - y as i32){
                            counter_1 += 1;
                        }
                    }
                }
            }
        }
    }


    let mut counter_2 = 0;

    // Part 2
    for (x, _) in lines.iter().enumerate(){
        for (y, _) in lines[x].char_indices(){
            if lines[x].chars().nth(y).unwrap() == 'A' && x > 0 && x < lines.len() - 1 && y > 0{
                let up = vec![lines[x - 1].get(y - 1..y).unwrap_or("L"), lines[x - 1].get(y+1..y+2).unwrap_or("L")];
                let down = vec![lines[x + 1].get(y - 1..y).unwrap_or("L"), lines[x + 1].get(y+1..y+2).unwrap_or("L")];
                if up == vec!["M", "M"] && down == vec!["S", "S"] ||
                   up == vec!["M", "S"] && down == vec!["M", "S"] ||
                   up == vec!["S", "M"] && down == vec!["S", "M"] ||
                   up == vec!["S", "S"] && down == vec!["M", "M"]
                {
                    counter_2 += 1;
                }
            }
        }
    }

    let start = time::Instant::now();
    println!("{:?}", counter_1);
    println!("{:?}", counter_2);
    let end = start.elapsed();
    println!("Temps écoulé : {:?}", end);

}
