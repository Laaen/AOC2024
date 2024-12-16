use std::{fs, ops::SubAssign};

enum Tile{
    
}

#[derive(Debug)]
#[derive(Clone)]
struct Storage{
    data: Vec<Vec<char>>,
    robot: (i32, i32)
}

impl Storage{
    fn from(data: &str) -> Self{

        let data: Vec<Vec<char>> = data.split("\r\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

        let mut robot = (0, 0);

        for x in 0..data.len(){
            for y in 0..data[0].len(){
                if data[x][y] == '@'{
                    robot = (x as i32, y as i32);
                }
            }
        }

        return Self{data, robot};
    }

    fn try_move_robot(&mut self, direction: char){
        let next_tile = match direction{
            '<' => (self.robot.0, self.robot.1 - 1),
            '^' => (self.robot.0 - 1, self.robot.1),
            '>' => (self.robot.0, self.robot.1 + 1),
            'v' => (self.robot.0 + 1, self.robot.1),
            _ => {(0, 0)}
        };
        
        match self.data[next_tile.0 as usize][next_tile.1 as usize]{
            '#' => { /* Rien */}
            '.' => { 
                self.data[next_tile.0 as usize][next_tile.1 as usize] = '@';
                self.data[self.robot.0 as usize][self.robot.1 as usize] = '.';
                self.robot = next_tile;
            }
            'O' => {
                if self.try_move_crate(next_tile, direction){
                    self.data[next_tile.0 as usize][next_tile.1 as usize] = '@';
                    self.data[self.robot.0 as usize][self.robot.1 as usize] = '.';
                    self.robot = next_tile;
                }
            }
            _ => {}
        }
    }

    fn get(&self, coords: (i32, i32)) -> char{
        return self.data[coords.0 as usize][coords.1 as usize];
    }


    // Tries to move a crate returns true if successful
    fn try_move_crate(&mut self, coords: (i32, i32), direction: char) -> bool{
        let next_tile = match direction{
            '<' => (coords.0, coords.1 - 1),
            '^' => (coords.0 - 1, coords.1),
            '>' => (coords.0, coords.1 + 1),
            'v' => (coords.0 + 1, coords.1),
            _ => {return false;}
        };

        match self.data[next_tile.0 as usize][next_tile.1 as usize]{
            '#' => { return false;}
            '.' => { 
                // Move the crate
                self.data[next_tile.0 as usize][next_tile.1 as usize] = 'O';
                self.data[coords.0 as usize][coords.1 as usize] = '.';
                return true;
            }
            'O' => {
                if self.try_move_crate(next_tile, direction){
                    // Move the crate
                    self.data[next_tile.0 as usize][next_tile.1 as usize] = 'O';
                    self.data[coords.0 as usize][coords.1 as usize] = '.';
                    return true;
                }
            }
            _ => {}
        }

        return false;
    }


    fn gps_sum(&self) -> i32{
        let mut res = 0;
        for x in 0..self.data.len(){
            for y in 0..self.data[0].len(){
                if self.data[x][y] == 'O'{
                    res += 100 * x + y;
                }
            }
        }

        return res as i32;
    }

    fn gps_sum_bis(&self) -> i32{
        let mut res = 0;
        for x in 0..self.data.len(){
            for y in 0..self.data[0].len(){
                if self.data[x][y] == '['{
                    res += 100 * x + y;
                }
            }
        }

        return res as i32;
    }

    fn transform_p2(&mut self){
        let mut new_data: Vec<Vec<char>> = Vec::new();
        for line in self.data.iter(){
            let mut tmp: Vec<char> = Vec::new();
            for chr in line{
                match chr{
                    '.' => {
                        tmp.push('.');
                        tmp.push('.');
                    },
                    '#' => {
                        tmp.push('#');
                        tmp.push('#');
                    },
                    'O' => {
                        tmp.push('[');
                        tmp.push(']');
                    },
                    '@' => {
                        tmp.push('@');
                        tmp.push('.');
                    }
                    _ => {}
                }
            }
            new_data.push(tmp);
        }

        self.data = new_data;

        let mut robot = (0, 0);

        for x in 0..self.data.len(){
            for y in 0..self.data[0].len(){
                if self.data[x][y] == '@'{
                    robot = (x as i32, y as i32);
                }
            }
        }

        self.robot = robot;

    }

}

impl std::fmt::Display for Storage{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\r\n");
        write!(f, "{}", data)
    }
}

fn main() {
    let data = fs::read_to_string("input").unwrap();

    let tmp: Vec<&str> = data.split("\r\n\r\n").collect();

    let grid = tmp[0];
    let path = tmp[1].replace("\r\n", "");

    let mut storage = Storage::from(grid);
    let mut storage_2 = storage.clone();
    for elt in path.chars(){
        storage.try_move_robot(elt);
    }

    println!("{}", storage.gps_sum());

}
