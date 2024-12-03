use regex::{Regex, RegexBuilder};
use std::fs;

fn part_1(data : &String) -> i32{
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re_mul.captures_iter(&data)
    .map(|c| c.extract())
    .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
    .fold(0, |res, x| res + x)
}

fn part_2(data : &String) -> i32{
    let re_suppr = RegexBuilder::new(r"don\'t\(\)(.*?)(?:do\(\)|$)").build().unwrap();
    let new_data = re_suppr.replace_all(&data, "");
    println!("{}", new_data);
    part_1(&new_data.into_owned())
}

fn main() {

    let data = fs::read_to_string("input").unwrap();

    let res_part_1 = part_1(&data);
    let res_part_2 = part_2(&data.replace("\n", ""));

    println!("{:?}", res_part_1);
    println!("{:?}", res_part_2);

}
