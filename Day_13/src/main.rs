use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Machine{
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

impl Machine{
    fn solve(&self) -> Option<(i64, i64)>{
        let possible_y = (self.prize.1 * self.button_a.0 - self.prize.0 * self.button_a.1)/(self.button_b.1 * self.button_a.0 - self.button_b.0 * self.button_a.1);
        let possible_x = (self.prize.0 - self.button_b.0 * possible_y) / self.button_a.0;

        // Check for strict equality
        if (possible_x * self.button_a.0 + possible_y * self.button_b.0 == self.prize.0) && 
           (possible_x * self.button_a.1 + possible_y * self.button_b.1 == self.prize.1){
            return Some((possible_x, possible_y));
        }
        return None;
    }

    fn set_part2(&mut self){
        self.prize = (self.prize.0 + 10000000000000, self.prize.1 + 10000000000000);
    }

}


fn main() {
    let data = fs::read_to_string("input").unwrap();

    let re = Regex::new(r"A: X([+-]\d+), Y([+-]\d+).*?B: X([+-]\d+), Y([+-]\d+).*?X=(\d+), Y=(\d+)").unwrap();
    let mut machines: Vec<Machine> = Vec::new();

    for elt in  data.split("\r\n\r\n"){
        for (_, [ax, ay, bx, by, tx, ty]) in re.captures_iter(&elt.replace("\r\n", "")).map(|c| c.extract()){
            let button_a: (i64, i64) = (ax.parse().unwrap(), ay.parse().unwrap());
            let button_b: (i64, i64) = (bx.parse().unwrap(), by.parse().unwrap());
            let prize: (i64, i64) = (tx.parse::<i64>().unwrap(), ty.parse::<i64>().unwrap());
            machines.push(Machine{button_a, button_b, prize});

        }
    }

    let mut tokens_p1 = 0;
    let mut tokens_p2 = 0;
    for m in machines.iter_mut(){
        if let Some((x, y)) = m.solve(){
            if x <= 100 && y <= 100{
                tokens_p1 += x * 3 + y;
            }
        }
        m.set_part2();
        if let Some((x, y)) = m.solve(){
            tokens_p2 += x * 3 + y;
        }
    }

    println!("{:?}", tokens_p1);
    println!("{:?}", tokens_p2);

}
