use std::fs;

#[derive(Debug)]
enum OpCode{
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv, 
    cdv
}

impl From<u64> for  OpCode{
    fn from(value: u64) -> Self {
        match value {
            0 => OpCode::adv,
            1 => OpCode::bxl,
            2 => OpCode::bst,
            3 => OpCode::jnz,
            4 => OpCode::bxc,
            5 => OpCode::out,
            6 => OpCode::bdv,
            7 => OpCode::cdv,
            _ => panic!("Unknown instruction")
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct VM{
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    IP: u64,
}

impl VM{
    fn init(a: u64, b: u64, c: u64) -> Self{
        Self{reg_a: a, reg_b: b, reg_c: c, IP: 0}
    }

    fn reset(&mut self,a: u64){
        self.reg_a = a;
        self.reg_b = 0;
        self.reg_c = 0;
        self.IP = 0;
    }

    fn combo_operand(&self, operand: u64) -> u64{
        match operand{
            0..4 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Incorrect operand")
        }
    }

    fn interpret(&mut self, instructions: &Vec<u64>) -> Vec<u64>{
        let mut stdout: Vec<u64> = Vec::new();
        while self.IP < (instructions.len() - 1) as u64{
            match OpCode::from(instructions[self.IP as usize]){
                OpCode::adv => {
                    self.reg_a /=  2_u64.pow(self.combo_operand(instructions[(self.IP + 1) as usize]) as u32);
                    self.IP += 2;
                    
                }
                OpCode::bxl => {
                    self.reg_b ^= instructions[(self.IP + 1) as usize];
                    self.IP += 2;
                }
                OpCode::bst => {
                    self.reg_b = self.combo_operand(instructions[(self.IP + 1) as usize]) % 8;
                    self.IP += 2;
                }
                OpCode::jnz => {
                    if self.reg_a == 0{
                        self.IP += 2;
                    } else {
                        self.IP = instructions[(self.IP + 1) as usize];
                    }
                }
                OpCode::bxc => {
                    self.reg_b ^= self.reg_c;
                    self.IP += 2;
                }
                OpCode::out => {
                    stdout.push(self.combo_operand(instructions[(self.IP + 1) as usize]) % 8);
                    self.IP += 2;
                }
                OpCode::bdv => {
                    self.reg_b = self.reg_a /  2_u64.pow(self.combo_operand(instructions[(self.IP + 1) as usize]) as u32);
                    self.IP += 2;
                }
                OpCode::cdv => {
                    self.reg_c = self.reg_a /  2_u64.pow(self.combo_operand(instructions[(self.IP + 1) as usize]) as u32);
                    self.IP += 2;
                }
                _ => {}
            }
        }

        return stdout;
    }
}

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let tmp: Vec<&str> = data.split("\n\n").collect();
    let registers: Vec<u64> = tmp[0].split("\n").map(|line| line.split(": ").collect::<Vec<&str>>().last().unwrap().parse::<u64>().unwrap()).collect();

    let mut vm = VM::init(registers[0], registers[1], registers[2]);
    let instructions: Vec<u64> = tmp[1].split(": ").collect::<Vec<&str>>()[1].split(",").map(|elt| elt.parse::<u64>().unwrap()).collect();

    let part_1 = vm.interpret(&instructions);
    println!("{}", part_1.iter().map(|elt| elt.to_string()).collect::<Vec<String>>().join(","));

    let expected = [2,4,1,3,7,5,0,3,1,5,4,4,5,5,3,0];

    let mut test = 0o3;
    test *= 0o10;

    let mut a: u64 = 0o0;
    let mut i: i32 = (expected.len() - 1) as i32;

    while i >= 0{
        vm.reset(a);
        while vm.interpret(&instructions) != &expected[i as usize..expected.len()] {
            a += 0o1;
            vm.reset(a);
        }
        vm.reset(a);
        if vm.interpret(&instructions) == &expected[i as usize..expected.len()] && i == 0{
            println!("{}", a);
        }
        a *= 0o10;
        i -= 1;
    }

    //println!("{}", a);

}
