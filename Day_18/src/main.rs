use std::i32;
use std::ops;
use std::fs;
use std::time::Instant;

enum Direction{
    North,
    South, 
    East, 
    West
}

impl Direction{
    fn value(&self) -> (i32, i32){
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

impl ops::Add<&Direction> for (i32, i32){
    type Output = (i32, i32);
    fn add(self, rhs: &Direction) -> Self::Output {
        (self.0 + rhs.value().0, self.1 + rhs.value().1)
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Vertex{
    coords: (i32, i32),
    value: i32
}

fn generate_vertices_list(height: i32, width: i32, corrupted_coords: &[(i32, i32)]) -> Vec<Vertex>{
    let mut vertices_list: Vec<Vertex> = Vec::new();

    for x in 0..height + 1{
        for y in 0..width + 1{
            if x == 0 && y == 0{
                vertices_list.push(Vertex{coords: (x, y), value: 0});
            }
            if !corrupted_coords.contains(&(x, y)){
                vertices_list.push(Vertex{coords: (x, y), value: i32::MAX});
            }
        }
    }

    return vertices_list;
}

fn solve(vertices_list: &Vec<Vertex>, end_coords: (i32, i32)) -> Option<i32>{

    let mut vertices_list = vertices_list.clone();

    let mut visited: Vec<Vertex> = vec![];

    let directions = vec![Direction::North, Direction::South, Direction::East, Direction::West];

    vertices_list.sort_by(|a, b| b.value.cmp(&a.value));

    let mut i: i32 = vertices_list.len() as i32 - 1;

    while i >= 0{
        let current= vertices_list[i as usize];
        for d in directions.iter(){
            let new_coords = vertices_list[i as usize].coords + d;
            if let Some(v) = vertices_list.iter_mut().filter(|vert| vert.coords == new_coords).collect::<Vec<&mut Vertex>>().first_mut(){
                if current.value == i32::MAX{
                    break;
                }
                v.value = current.value + 1;

            }
        } 

        visited.push(vertices_list[i as usize]);
        vertices_list.remove(i as usize);
        vertices_list.sort_by(|a, b| b.value.cmp(&a.value));
        i -= 1;
    }

    if let Some(res) = visited.iter().filter(|v| v.coords == end_coords && v.value != i32::MAX).collect::<Vec<&Vertex>>().first(){
        return Some(res.value);
    } else {
        return None;
    }
}

fn main() {

    let start_time = Instant::now();
    
    let data = fs::read_to_string("input").unwrap();
    let corrupted_coords = &data.split("\r\n")
    .map(|line| {
        let splitted_line: Vec<&str> = line.split(",").collect();
        (splitted_line[1].parse().unwrap(), splitted_line[0].parse().unwrap())
    })
    .collect::<Vec<(i32, i32)>>();

    let res_1 = solve(&generate_vertices_list(70, 70, &corrupted_coords[0..1024]), (70, 70)).unwrap();
    println!("{}", res_1);

    let mut start = 1024;
    let mut end = corrupted_coords.len();
    let mut i = 0;
    while end - start > 1{
        i = (end + start) / 2;
        if let None = solve(&generate_vertices_list(70, 70, &corrupted_coords[0..i]), (70, 70)){
            end = i;
        } else {
            start = i;
        }
    }
    println!("{},{}", corrupted_coords[i - 1].1, corrupted_coords[i - 1].0);
    println!("{:?}", Instant::now() - start_time);
}
