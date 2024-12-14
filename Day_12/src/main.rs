use std::fs;
use std::collections::{HashMap, HashSet};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// 2 identical and 2 different (or not present)
const OUTER_CORNERS: [[(i32, i32); 4]; 4] = [[(-1, 0), (0, -1), (1, 0), (0, 1)],
                                       [(-1, 0), (0, 1), (1, 0), (0, -1)],
                                       [(1, 0), (0, 1), (-1, 0), (0, -1)],
                                       [(1, 0), (0, -1), (-1, 0), (0, 1)]];

const INNER_CORNERS: [[(i32, i32); 3]; 4] = [[(-1, 0), (0, 1), (-1, 1)],
                                             [(-1, 0), (0, -1), (-1, -1)],
                                             [(1, 0), (0, 1), (1, 1)],
                                             [(1, 0), (0, -1), (1, -1)]];

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
struct Region{
    category: char,
    plants: Vec<(i32, i32)>
}

impl Region {
    fn size(&self) -> i32{
        self.plants.len() as i32
    }

    fn perimeter(&self) -> i32{
        let mut perimeter = 0;
        for elt in self.plants.iter(){
            let mut curr = 4;
            for d in DIRECTIONS{
                if self.plants.contains(&(d.0 + elt.0, d.1 + elt.1)){
                    curr -= 1;
                }
            }
            perimeter += curr;
        }

        return perimeter
    }

    fn price(&self) -> i32{
        self.perimeter() * self.size()
    }

    fn price_sides(&self) -> i32{
        self.sides() * self.size()
    }

    fn get_nb_neighbours(&self, x: i32, y: i32) -> i32{
        let mut neighbours = 0;
        for d in DIRECTIONS{
            if self.plants.contains(&(d.0 + x, d.1 + y)){
                neighbours += 1;
            }
        }
        return neighbours;
    }

    fn get_outer_corner(&self, x: i32, y: i32) -> i32{
        let mut res = 0;
        for c in OUTER_CORNERS{
            if self.plants.contains(&(x + c[0].0, y + c[0].1)) &&
               self.plants.contains(&(x + c[1].0, y + c[1].1)) &&
               !self.plants.contains(&(x + c[2].0, y + c[2].1)) &&
               !self.plants.contains(&(x + c[3].0, y + c[3].1)){
                res += 1;
            }
        }
        return res;
    }

    fn get_inner_corner(&self, x: i32, y: i32) -> i32{
        let mut res = 0;
        for c in INNER_CORNERS{
            if self.plants.contains(&(x + c[0].0, y + c[0].1)) &&
               self.plants.contains(&(x + c[1].0, y + c[1].1)) &&
               !self.plants.contains(&(x + c[2].0, y + c[2].1)){
                res += 1;
            }
        }
        return res;
    }

    fn sides(&self) -> i32{
        let mut sides = 0;

        // If only a line or single plant
        if self.plants.len() == 1{
            return 4;
        }
        if self.plants.iter().all(|x| x.0 == self.plants[0].0) || self.plants.iter().all(|x| x.1 == self.plants[0].1){
            return 4;
        }

        for elt in self.plants.iter(){
            if self.get_nb_neighbours(elt.0, elt.1) == 1{
                sides += 2;
            }

            sides += self.get_outer_corner(elt.0, elt.1);
            sides += self.get_inner_corner(elt.0, elt.1);
        }

        return sides;
    }

}

fn get_char_at(data: &Vec<Vec<char>>, x: usize, y: usize) -> Option<&char>{
    data.get(x)?.get(y)
}

fn get_region(data: &Vec<Vec<char>>, x: usize, y: usize, acc: &mut Vec<(usize, usize)>){
    
    acc.push((x, y));
    

    let current_char = get_char_at(&data, x, y).unwrap();
    for d in DIRECTIONS{
        let new_x = (x as i32 + d.0) as usize;
        let new_y = (y as i32 + d.1) as usize;
        if get_char_at(&data, new_x, new_y).unwrap_or(&'0') == current_char && !acc.contains(&(new_x, new_y)){
            get_region(&data, new_x, new_y, acc);
        }
    }
}

fn main() {
    let data = fs::read_to_string("input").unwrap();

    let grid: Vec<Vec<char>> = data.split("\r\n")
    .map(|line| line.chars().collect())
    .collect();

    let mut regions: HashMap<char, HashSet<Region>> = HashMap::new();

    for x in 0..grid.len(){
        for y in 0..grid.len(){
            let chr = get_char_at(&grid, x, y).unwrap();
            let mut acc: Vec<(usize, usize)> = Vec::new();
            get_region(&grid, x, y, &mut acc);
            acc.sort();
            let reg = Region{category: *chr, plants: acc.iter().map(|x| (x.0 as i32, x.1 as i32)).collect()};
            if regions.contains_key(chr){
                regions.get_mut(chr).unwrap().insert(reg);
            } else {
                let mut tmp: HashSet<Region> = HashSet::new();
                tmp.insert(reg);
                regions.insert(*chr, tmp);
            }
        }
    }

    let mut res_1 = 0;
    let mut res_2 = 0;

    for (_, v) in regions{
        for reg in v{
            res_1 += reg.price();
            res_2 += reg.price_sides();
        }
    }

    println!("{res_1}");
    println!("{res_2}");

}
