use std::{cmp::min, fs, ops};

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
enum Direction{
    North,
    South,
    East,
    West,
}

impl Direction {
    fn value(&self) -> (i32, i32){
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1)
        }
    }

    fn opposite(&self) -> Direction{
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

}

impl ops::Add<&Direction> for (i32, i32){
    type Output = (i32, i32);
    fn add(self, rhs: &Direction) -> (i32, i32) {
        let dir = rhs.value();
        return (dir.0 + self.0, dir.1 + self.1)
    }
}

#[derive(Debug)]
enum Tile{
    Empty,
    Wall,
    Start,
    End
}

impl From<char> for Tile{
    fn from(value: char) -> Self {
        match value{
            '.' => Self::Empty,
            '#' => Self::Wall,
            'E' => Self::End,
            'S' => Self::Start,
            _ => panic!("Got unknown char")
        }
    }
}

impl From<&Tile> for char{
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Start => 'S',
            Tile::End => 'E'
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Reindeer{
    visited: Vec<(i32, i32)>,
    position: (i32, i32),
    direction: Direction,
    current_score: i32,
}

#[derive(Debug)]
struct Maze{
    data: Vec<Vec<Tile>>,
}

impl std::fmt::Display for Maze{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.iter().map(|line| line.iter().map(|elt| char::from(elt)).collect::<String>()).collect::<Vec<String>>().join("\r\n"))
    }
}

impl Maze{
    fn from(input: &str, sep: &str) -> Self{
        let mut data: Vec<Vec<Tile>> = Vec::new();
        for line in input.split(sep){
            let mut tmp: Vec<Tile> = Vec::new();
            for chr in line.chars(){
                tmp.push(Tile::from(chr));
            }
            data.push(tmp);
        }
        return Self{data}
    }

    fn get(&self, coords: (i32, i32)) -> &Tile{
        return &self.data[coords.0 as usize][coords.1 as usize];
    }

    fn next_move(&self, r: &mut Reindeer, scores: &mut Vec<i32>) -> i32{
        let directions = [Direction::North, Direction::South, Direction::East, Direction::West];

        for d in directions{
            if d != r.direction.opposite(){
                match self.get(r.position + &d){
                    Tile::Empty => {
                        if !r.visited.contains(&(r.position + &d)){
                            let mut new_r = r.clone();
                            new_r.position = r.position + &d;
                            new_r.visited.push(r.position + &d);
                            if d == new_r.direction{
                                new_r.current_score += 1;
                            } else {
                                new_r.current_score += 1001;
                            }
                            new_r.direction = d;
                            self.next_move(&mut new_r, scores);
                        }
                    }
                    Tile::Wall => {/* Nothing */}
                    Tile::End => {
                        if d == r.direction{
                            scores.push(r.current_score + 1);
                        } else {
                            scores.push(r.current_score + 1001);
                        }
                    }
                    Tile::Start => {}
                }
            }
        }

        return min(r.current_score, 7036);
    }

    fn solve(&self) -> i32{
        let mut scores: Vec::<i32> = Vec::new();
        let res = self.next_move(&mut Reindeer{position: (13, 1), current_score: 0, direction: Direction::East, visited: vec![]}, &mut scores);
        scores.iter().min().unwrap().clone()
    }
}

fn main() {

    let data = fs::read_to_string("input").unwrap();
    let maze = Maze::from(&data, "\r\n");

    println!("{}", maze.solve());

}
