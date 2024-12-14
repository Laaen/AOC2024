use std::fs;
use regex::Regex;

const HEIGHT: i32 = 103;
const WIDTH: i32 = 101;

#[derive(Debug)]
#[derive(Clone)]
struct Robot{
    position: (i32, i32),
    vector: (i32, i32)
}

impl Robot{
    fn move_after(&mut self, seconds: i32){
        let new_x = (self.position.0 + seconds * self.vector.0).rem_euclid(HEIGHT);
        let new_y = (self.position.1 + seconds * self.vector.1).rem_euclid(WIDTH);
        
        self.position = (new_x, new_y);

    }
}

struct Grid{
    height: usize,
    width: usize,
    robots: Vec<(i32, i32)>
}

impl Grid {

    fn display(&self) -> String{
        let mut grid: Vec<Vec<char>> = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];
        for x in 0..self.height{
            for y in 0..self.width{
                if self.robots.contains(&(x as i32, y as i32)){
                    grid[x][y] = 'X';
                }
            }
        }
        grid.iter().map(|line| String::from_iter(line)).collect::<Vec<String>>().join("\r\n")
    }

    fn check_tree(&self) -> bool{
        for y in 0..self.width{
            let mut col = self.robots.iter().filter(|r| r.1 == y as i32).collect::<Vec<_>>();
            col.sort();
            let mut consec = 0;
            for elt in col.iter().map(|x| x.0).collect::<Vec<_>>().windows(2){
                if elt[1] == elt[0] + 1{
                    consec += 1;
                }
            }
            if consec >= 30{
                return true;
            }
        }
        return false;
    }
    
}

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let reg = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<Robot> = Vec::new();

    for (_, [py, px, vy, vx]) in reg.captures_iter(&data.replace("\r\n", "")).map(|c| c.extract()){
        robots.push(Robot{position: (px.parse().unwrap(), py.parse().unwrap()), vector: (vx.parse().unwrap(), vy.parse().unwrap())});
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for r in robots.clone().iter_mut(){
        r.move_after(100);
        // Dispatch in quadrants
        if r.position.0 != HEIGHT / 2 && r.position.1 != WIDTH / 2{
            if r.position.0 < HEIGHT / 2 {
                if r.position.1 < WIDTH / 2{
                    q1 += 1;
                } else {
                    q2 += 1;
                }
            } else {
                if r.position.1 < WIDTH / 2{
                    q3 += 1;
                } else {
                    q4 += 1;
                }
            }
        }
    }

    println!("{}", q1 * q2 * q3 * q4);
    
    let mut robots_part_2 = robots.clone();
    let mut counter = 1;

    let mut found = false;

    while !found{
        for r in robots_part_2.iter_mut(){
            r.move_after(1);
        }
        let grid = Grid{height : HEIGHT as usize, width: WIDTH as usize, robots: robots_part_2.iter().map(|r| r.position).collect::<Vec<(i32, i32)>>()};

        if grid.check_tree(){
            found = true;
            println!("{}", grid.display());
            println!("{counter}");
        }
        counter += 1;
    }
}
