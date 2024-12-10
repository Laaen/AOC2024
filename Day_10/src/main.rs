use std::{collections::HashSet, fs};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn explore_trail_1(grid : &Vec<Vec<i32>>, coords: (usize, usize), value_to_find: i32) -> HashSet<(usize, usize)>{

    let mut result: HashSet<(usize, usize)> = HashSet::new();
    for d in DIRECTIONS{
        let x = (coords.0 as i32 + d.0) as usize;
        let y = (coords.1 as i32 + d.1) as usize;

        if grid.get(x).unwrap_or(&vec![]).get(y).unwrap_or(&0) == &value_to_find{
            if value_to_find == 9{
                result.insert((x, y));
            }
            result.extend(explore_trail_1(&grid, (x, y), value_to_find + 1));
        }
    }
    return result;
}

fn explore_trail_2(grid : &Vec<Vec<i32>>, coords: (usize, usize), value_to_find: i32) -> Vec<(usize, usize)>{

    let mut result: Vec<(usize, usize)> = Vec::new();
    for d in DIRECTIONS{
        let x = (coords.0 as i32 + d.0) as usize;
        let y = (coords.1 as i32 + d.1) as usize;

        if grid.get(x).unwrap_or(&vec![]).get(y).unwrap_or(&0) == &value_to_find{
            if value_to_find == 9{
                result.push((x, y));
            }
            result.extend(explore_trail_2(&grid, (x, y), value_to_find + 1));
        }
    }
    return result;
}

fn main() {

    let data = fs::read_to_string("input").unwrap();
    let grid : Vec<Vec<i32>> = data.split("\r\n")
    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(99) as i32).collect())
    .collect();

    let mut res_1: Vec<(usize, usize)> = Vec::new();
    let mut res_2: Vec<(usize, usize)> = Vec::new();
    for x in 0..grid.len(){
        for y in 0..grid[x].len(){
            // On a trouvé un 0, on suit le chemin
            if grid[x][y] == 0{
                res_1.extend(explore_trail_1(&grid, (x, y), 1));
                res_2.extend(explore_trail_2(&grid, (x, y), 1));
            }
        }
    }

    println!("{:?}", res_1.iter().count());
    println!("{:?}", res_2.iter().count());

}
