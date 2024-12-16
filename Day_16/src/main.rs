use std::hash::Hash;
use std::{borrow::BorrowMut, cmp::min, fs, i32, ops, thread::current};
use std::collections::{HashMap, HashSet};

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

#[derive(PartialEq)]
#[derive(Debug)]
enum Tile{
    Empty(i32),
    Wall,
    Start,
    End
}

impl From<char> for Tile{
    fn from(value: char) -> Self {
        match value{
            '.' => Self::Empty(i32::MAX),
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
            Tile::Empty(_) => '.',
            Tile::Wall => '#',
            Tile::Start => 'S',
            Tile::End => 'E'
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Reindeer{
    position: (i32, i32),
    direction: Direction,
    current_score: i32,
    visited: HashSet<(i32, i32)>
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

    fn next_move(&mut self, r: &mut Reindeer, scores: &mut Vec<i32>, used_tiles: &mut HashMap<i32, HashSet<(i32, i32)>>){
        let directions = [Direction::North, Direction::South, Direction::East, Direction::West];

        for d in directions{
            if d != r.direction.opposite(){
                match self.get(r.position + &d){
                    Tile::Empty(score) => {
                        if score >= &(r.current_score - 1000){
                            let mut new_r = r.clone();
                            new_r.position = r.position + &d;
                            new_r.visited.insert(r.position);
                            let c = r.position + &d;
                            if d == new_r.direction{
                                new_r.current_score = r.current_score + 1;
                                self.data[c.0 as usize][c.1 as usize] = Tile::Empty(new_r.current_score);
                            } else {
                                new_r.current_score = r.current_score + 1001;
                                self.data[c.0 as usize][c.1 as usize] = Tile::Empty(new_r.current_score);
                            }
                            new_r.direction = d;
                            self.next_move(&mut new_r, scores, used_tiles);
                        }
                    }
                    Tile::Wall => {/* Nothing */}
                    Tile::End => {
                        r.visited.insert(r.position + &d);
                        if d == r.direction{
                            r.current_score += 1;
                        } else {
                            r.current_score += 1001;
                        }
                        scores.push(r.current_score);
                        used_tiles.entry(r.current_score)
                        .and_modify(|hs| hs.extend(r.visited.iter()))
                        .or_insert(r.visited.clone());
                    }
                    Tile::Start => {}
                }
            }
        }
    }

    fn solve(&mut self) -> (i32, i32){
        // Get the starting position
        let mut start = (0, 0);
        for x in 0..self.data.len(){
            for y in 0..self.data[0].len(){
                if self.get((x as i32, y as i32)) == &Tile::Start{
                    start = (x as i32, y as i32);
                }
            }
        }
        let mut scores: Vec::<i32> = Vec::new();
        let mut visited_tiles: HashMap::<i32, HashSet<(i32, i32)>> = HashMap::new();
        let mut visited = HashSet::new();
        self.next_move(&mut Reindeer{position: start, current_score: 0, direction: Direction::East, visited: visited}, &mut scores, &mut visited_tiles);
        (scores.iter().min().unwrap().clone(), visited_tiles[scores.iter().min().unwrap()].len() as i32)
    }
}

fn main() {

    let data = fs::read_to_string("input").unwrap();
    let mut maze = Maze::from(&data, "\r\n");

    let res = maze.solve();

    println!("Part 1 : {}   | Part 2 : {}", res.0, res.1 + 1);

}
