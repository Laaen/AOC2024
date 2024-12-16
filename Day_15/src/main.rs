use std::{fs, ops::{self, SubAssign}};

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Direction{
    North,
    South,
    East,
    West
}

impl Direction{
    fn value(&self) -> (i32, i32){
        match self{
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1)
        }
    }
}

impl From<char> for Direction{
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::North,
            'v' => Direction::South,
            '<' => Direction::West,
            '>' => Direction::East,
            _ => panic!("Unknown direction")
        }
    }
}

impl ops::Add<Direction> for (i32, i32){
    type Output = (i32, i32);
    fn add(self, rhs: Direction) -> Self::Output {
        (self.0 + rhs.value().0, self.1 + rhs.value().1)
    }
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Tile{
    Wall,
    Box,
    Empty,
    Robot,
    BoxLeft,
    BoxRight
}

impl Tile{
    fn biggerify(&self) -> [Tile; 2]{
        match self {
            Tile::Wall => [Tile::Wall, Tile::Wall],
            Tile::Empty => [Tile::Empty, Tile::Empty],
            Tile::Robot => [Tile::Robot, Tile::Empty],
            Tile::Box => [Tile::BoxLeft, Tile::BoxRight],
            _ => panic!("Unknown")
        }
    }
}

impl From<char> for Tile{
    fn from(value: char) -> Self {
        match value{
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '.' => Tile::Empty,
            '@' => Tile::Robot,
            '[' => Tile::BoxLeft,
            ']' => Tile::BoxRight,
            _ => panic!("Unknown char")
        }
    }
}

impl From<&Tile> for char{
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Box => 'O',
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Robot => '@',   
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']'
        }
    }
}

struct Warehouse{
    data: Vec<Vec<Tile>>,
    current_robot_position: (i32, i32),
}

impl Clone for Warehouse{
    fn clone(&self) -> Self {
        let data = self.data.clone();
        let pos = self.current_robot_position.clone();
        Self{data: data, current_robot_position: pos}
    }
}

impl Warehouse{
    fn from(input: &str, sep: &str) -> Self{
        let mut data: Vec<Vec<Tile>> = Vec::new();
        // Build grid
        for line in input.split(sep){
            let mut tmp: Vec<Tile> = Vec::new();
            for chr in line.chars(){
                tmp.push(Tile::from(chr));
            }
            data.push(tmp);
        }

        Self{data, current_robot_position: (0, 0)}
    }

    fn get(&self, coords: (i32, i32)) -> &Tile{
        &self.data[coords.0 as usize][coords.1 as usize]
    }

    fn can_move(&self, coords: (i32, i32), direction: Direction) -> bool{
        let new_coords = coords + direction;
        match self.get(new_coords) {
            Tile::Wall => false,
            Tile::Box => self.can_move(new_coords, direction),
            Tile::Empty => true,
            Tile::Robot => false,
            Tile::BoxLeft => {
                if direction == Direction::North || direction == Direction::South{
                    self.can_move(new_coords, direction) && self.can_move((new_coords.0, new_coords.1 + 1), direction)
                } else {
                    self.can_move(new_coords, direction)
                }
            }
            Tile::BoxRight => {
                if direction == Direction::North || direction == Direction::South{
                    self.can_move(new_coords, direction) && self.can_move((new_coords.0, new_coords.1 - 1), direction)
                } else {
                    self.can_move(new_coords, direction)
                }
            }
        }
    }

    fn move_tile(&mut self, coords: (i32, i32), direction: Direction){
        let new_coords = coords + direction;
        match self.get(new_coords) {
            Tile::Wall => panic!("Got in a wall while moving !"),
            Tile::Box => {
                self.move_tile(new_coords, direction);
                self.data[new_coords.0 as usize][new_coords.1 as usize] = self.data[coords.0 as usize][coords.1 as usize];
                self.data[coords.0 as usize][coords.1 as usize] = Tile::Empty;
            },
            Tile::Empty => {
                self.data[new_coords.0 as usize][new_coords.1 as usize] = self.data[coords.0 as usize][coords.1 as usize];
                self.data[coords.0 as usize][coords.1 as usize] = Tile::Empty;
            },
            Tile::Robot => panic!("Got into itself @_@"),
            Tile::BoxLeft => {
                if direction == Direction::North || direction == Direction::South{
                    self.move_tile((new_coords.0, new_coords.1 + 1), direction);
                }
                self.move_tile(new_coords, direction);
                self.data[new_coords.0 as usize][new_coords.1 as usize] = self.data[coords.0 as usize][coords.1 as usize];
                self.data[coords.0 as usize][coords.1 as usize] = Tile::Empty;
            }
            Tile::BoxRight => {
                if direction == Direction::North || direction == Direction::South{
                    self.move_tile((new_coords.0, new_coords.1 - 1), direction);
                }
                self.move_tile(new_coords, direction);
                self.data[new_coords.0 as usize][new_coords.1 as usize] = self.data[coords.0 as usize][coords.1 as usize];
                self.data[coords.0 as usize][coords.1 as usize] = Tile::Empty;
            }
        };

    }

    fn move_robot(&mut self, direction: char){
        let dir = Direction::from(direction);

        if(self.can_move(self.current_robot_position, dir)){
            self.move_tile(self.current_robot_position, dir);
            self.current_robot_position = self.current_robot_position + dir;
        }
    }

    fn gps_coord(&self, symbol: char) -> i32{
        let mut sum = 0;
        for x in 0..self.data.len(){
            for y in 0..self.data[0].len(){
                if self.data[x][y] == Tile::from(symbol){
                    sum += 100 * x + y;
                }
            }
        }
        return sum as i32;
    }

    fn biggerify(&mut self){
        let mut new_data: Vec<Vec<Tile>> = Vec::new();
        for line in self.data.clone(){
            let mut tmp: Vec<Tile> = Vec::new();
            for t in line{
                tmp.extend(t.biggerify());
            }
            new_data.push(tmp);
        }
        self.data = new_data;
    }

    fn set_start_pos(&mut self){
        let mut start_pos = (0, 0);
        for x in 0..self.data.len(){
            for y in 0..self.data[0].len(){
                if self.data[x][y] == Tile::Robot{
                    start_pos = (x as i32, y as i32);
                }
            }
        }
        self.current_robot_position = start_pos;
    }

}

impl std::fmt::Display for Warehouse{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.iter().map(|line| line.iter().map(|v| char::from(v)).collect::<String>()).collect::<Vec<String>>().join("\r\n"))
    }
}

fn main() {
    let data = fs::read_to_string("input").unwrap();

    let tmp: Vec<&str> = data.split("\r\n\r\n").collect();

    let mut warehouse = Warehouse::from(&tmp[0], "\r\n");

    let mut w1 = warehouse.clone();
    w1.set_start_pos();

    for chr in tmp[1].replace("\r\n", "").chars(){
       w1.move_robot(chr);
    }

    println!("{}", &w1.gps_coord('O'));

    warehouse.biggerify();
    let mut w2 = warehouse.clone();
    w2.set_start_pos();

    for chr in tmp[1].replace("\r\n", "").chars(){
        w2.move_robot(chr);
     }
    println!("{}", &w2.gps_coord('['));
    
}
