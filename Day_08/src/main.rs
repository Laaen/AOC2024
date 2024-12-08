use std::{collections::{HashMap, HashSet}, fs};
use std::time;

/// Checks if the given coord is within bounds of the grid
fn check_inbound(value : (i32, i32), x : usize, y : usize) -> bool{
    return value.0 >= 0 && value.0 < x as i32 && value.1 >= 0 && value.1 < y as i32;
}

fn main() {
    let start = time::Instant::now();
    let data = fs::read_to_string("input").expect("File input should exist");

    let map : Vec<&str> = data.split("\r\n").collect();
    let mut antennas : HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    // Get coords of antennas
    for (i, _) in map.iter().enumerate(){
        for (j, chr) in map[i].chars().enumerate(){
            if chr.is_alphanumeric(){
                antennas.entry(chr).and_modify(|val| val.push((i as i32, j as i32))).or_insert(vec![(i as i32, j as i32)]);
            }
        }
    }

    // Part 1
    let mut result_1 : HashSet<(i32, i32)> = HashSet::new(); 

    for (_, v) in antennas.iter(){
        for (idx, elt) in v.iter().enumerate(){
            for elt_bis in v[idx + 1..v.len()].iter(){
                // Calc the coord and add them to the hashset
                let coord_1 = (2 * elt.0 - elt_bis.0, 2* elt.1 - elt_bis.1);
                let coord_2 = (2 * elt_bis.0 - elt.0, 2* elt_bis.1 - elt.1);
                if check_inbound(coord_1, map.len(), map[0].len()){
                    result_1.insert(coord_1);
                }
                if check_inbound(coord_2, map.len(), map[0].len()){
                    result_1.insert(coord_2);
                }
            }
        }
    }

    println!("{:?}", result_1.iter().count());

    // Part 2

    let mut result_2 : HashSet<(i32, i32)> = HashSet::new(); 

    for (_, v) in antennas.iter(){
        for (idx, elt) in v.iter().enumerate(){
            for elt_bis in v[idx + 1..v.len()].iter(){
                // Calc the coord and add them to the hashset
                let mut i = 1;
                let mut coord_1= (0,0);
                let mut coord_2= (0,0);
                result_2.insert(*elt);
                result_2.insert(*elt_bis);
                while check_inbound(coord_1, map.len(), map[0].len()) || check_inbound(coord_2, map.len(), map[0].len()){
                    coord_1 = (elt.0 + (elt.0 - elt_bis.0) * i, elt.1 + (elt.1 - elt_bis.1) * i);
                    coord_2 = (elt_bis.0 + (elt_bis.0 - elt.0) * i, elt_bis.1 + (elt_bis.1 - elt.1) * i);
                    if check_inbound(coord_1, map.len(), map[0].len()){
                        result_2.insert(coord_1);
                    }
                    if check_inbound(coord_2, map.len(), map[0].len()){
                        result_2.insert(coord_2);
                    }
                    i += 1;
                }

            }
        }
    }
    println!("{:?}", result_2.iter().count());
    println!("{:?}", time::Instant::now() - start);
}
