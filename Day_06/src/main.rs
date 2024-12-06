use std::{collections::HashSet, fs};
use std::thread;
use std::sync::{Arc, Mutex};

fn get_char_at(data : &Vec<String>, x : usize, y: usize) -> char{
    let line = data.get(x);
    match line{
        Some(str) => {str.chars().nth(y).unwrap_or('X')}
        None => {'X'}
    }
}

fn main() {

    let data = fs::read_to_string("input").unwrap();
    let grid : Arc<Vec<String>> = Arc::new(data.split("\r\n").map(|elt| elt.to_string()).collect());

    // Starting position, used for both parts
    let start_pos : (usize, usize) = grid.iter()
    .enumerate()
    .filter(|(_idx, line)| line.contains("^"))
    .map(|(idx, line)| (idx, line.find("^").unwrap()))
    .collect::<Vec<(usize, usize)>>()[0];

    // ======================== Part 1 ========================

    // Get the current position
    let mut curr_pos = start_pos;

    // Visited positions
    let mut visited : HashSet<(usize, usize)> = HashSet::new();
    visited.reserve(10000);
    visited.insert(curr_pos);

    // Check every direction
    let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    for d in directions.iter().cycle(){
        while get_char_at(&grid, (curr_pos.0 as i32 + d.0) as usize, (curr_pos.1 as i32 + d.1) as usize)  != '#' &&
              get_char_at(&grid, (curr_pos.0 as i32 + d.0) as usize, (curr_pos.1 as i32 + d.1) as usize) != 'X'{
            curr_pos.0 = (curr_pos.0 as i32 + d.0) as usize;
            curr_pos.1 = (curr_pos.1 as i32 + d.1) as usize;
            visited.insert(curr_pos);
        }
        if get_char_at(&grid, (curr_pos.0 as i32 + d.0) as usize, (curr_pos.1 as i32 + d.1) as usize) == 'X'{
            break;
        }
    }

    println!("{:?}", visited.len());

    // ======================== Part 2 ========================
    let result = Arc::new(Mutex::new(0));
    
    // handles des threads
    let mut handles = vec![];


    // Bruteforce
    // Add a # on each position
    for (i, j) in visited{

        let result_cpy = result.clone();
        let mut grid_bis : Vec<String> = grid.clone().to_vec();

        let handle = thread::spawn(move || {
                if get_char_at(&grid_bis, i, j) == '.'{
                    grid_bis[i].replace_range(j..j+1, "#");
            
                    // Get the current position
                    let mut curr_pos = start_pos;
            
                    // Backtrace of already visidted coords
                    let mut visited : HashSet<(usize, usize)> = HashSet::new();
                    visited.reserve(10000);
                    let mut nb_identical_pos = 0;
            
                    // Check every direction
                    let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
            
                    for d in directions.iter().cycle(){
                        while get_char_at(&grid_bis, (curr_pos.0 as i32 + d.0) as usize, (curr_pos.1 as i32 + d.1) as usize)  != '#' &&
                            get_char_at(&grid_bis, (curr_pos.0 as i32 + d.0) as usize, (curr_pos.1 as i32 + d.1) as usize) != 'X'{
                            curr_pos.0 = (curr_pos.0 as i32 + d.0) as usize;
                            curr_pos.1 = (curr_pos.1 as i32 + d.1) as usize;
                            if visited.insert(curr_pos) == false{
                                nb_identical_pos += 1;
                            }
                        }
                        if nb_identical_pos >= 1000{
                            let mut nb = result_cpy.lock().unwrap();
                            *nb += 1;
                            break;
                        }
                        if get_char_at(&grid_bis, (curr_pos.0 as i32 + d.0) as usize, (curr_pos.1 as i32 + d.1) as usize) == 'X' {
                            break;
                        }
                    }
                    //println!("{:?}", grid_bis);
                    grid_bis[i].replace_range(j..j+1, ".");
                }
        });
        handles.push(handle);
    }

    for h in handles{
        h.join().unwrap();
    }

    println!("{:?}", result.lock().unwrap());
}
    
