use std::env;
use std::fs;
use std::io;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let part = env::args().nth(2).unwrap_or("2".to_string()).parse::<usize>().unwrap();

    let memory = parse(&input);
    let mut intcode = IntCode::new(&memory, part);
    intcode.run();
    println!("part {}: {}", part, intcode.damage);
}

fn parse(input: &str) -> Vec<i64> {
    let mut result = Vec::new();
    for c in input.trim().split(',') {
        result.push(c.parse::<i64>().unwrap());
    }
    result
}

enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    fn decode(n: i64) -> ParamMode {
        match n {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Unexpected parameter mode: {}", n),
        }
    }
}

fn decode(n: i64) -> (ParamMode, ParamMode, ParamMode, i64) {
    let de = n % 100;
    let c = ParamMode::decode(n / 100 % 10);
    let b = ParamMode::decode(n / 1000 % 10);
    let a = ParamMode::decode(n / 10000 % 10);
    (a, b, c, de)
}

struct IntCode {
    memory: Vec<i64>,
    ptr: usize,
    base: i64,
    input_buf: String,
    input_idx: usize,
    go: String,
    damage: u64,
}

impl IntCode {
    fn new(memory: &Vec<i64>, mode: usize) -> Self {
        IntCode {
            memory: memory.to_vec(),
            ptr: 0,
            base: 0,
            input_buf: String::new(),
            input_idx: 0,
            go: match mode {
                1 => String::from("WALK\n"),
                2 => String::from("RUN\n"),
                _ => panic!("unknown mode {}", mode),
            },
            damage: 0,
        }
    }

    fn read(&mut self, i: usize) -> i64 {
        if self.memory.len() <= i {
            self.memory.resize(i + 1, 0);
        }
        self.memory[i]
    }

    fn write(&mut self, i: usize, val: i64) {
        if self.memory.len() <= i {
            self.memory.resize(i + 1, 0);
        }
        self.memory[i] = val;
    }

    fn get(&mut self, i: usize, mode: ParamMode) -> i64 {
        match mode {
            ParamMode::Position => {
                let pos = self.read(i) as usize;
                self.read(pos)
            }
            ParamMode::Immediate => self.read(i),
            ParamMode::Relative => {
                let pos = self.read(i) + self.base;
                self.read(pos as usize)
            }
        }
    }

    fn set(&mut self, i: usize, mode: ParamMode, val: i64) {
        match mode {
            ParamMode::Position => {
                let pos = self.read(i) as usize;
                self.write(pos, val);
            }
            ParamMode::Immediate => self.write(i, val),
            ParamMode::Relative => {
                let pos = self.read(i) + self.base;
                self.write(pos as usize, val);
            }
        }
    }

    fn execute(&mut self) -> i64 {
        let i = self.ptr;
        let code = self.memory[i];
        let (arg3_mode, arg2_mode, arg1_mode, op) = decode(code);
        let next_i = match op {
            1 | 2 => {
                let val_1 = self.get(i + 1, arg1_mode);
                let val_2 = self.get(i + 2, arg2_mode);
                let res = match op {
                    1 => val_1 + val_2,
                    2 => val_1 * val_2,
                    _ => 0,
                };
                self.set(i + 3, arg3_mode, res);
                i + 4
            }
            3 => {
                // if input_buf is empty, get from stdin
                if self.input_buf.is_empty() {
                    let mut buffer = String::new();
                    while buffer != self.go {
                        buffer = String::new();
                        match io::stdin().read_line(&mut buffer) {
                            Err(error) => panic!("Error: {}", error),
                            Ok(_) => self.input_buf.push_str(&buffer),
                        }
                    }
                }
                let input_val = ((&self.input_buf[self.input_idx..self.input_idx+1]).parse::<char>().unwrap()
                    as u8) as i64;
                self.input_idx += 1;
                self.set(i + 1, arg1_mode, input_val);
                i + 2
            }
            4 => {
                let output_val = self.get(i + 1, arg1_mode);
                if output_val > u8::max_value() as i64 {
                    self.damage = output_val as u64;
                } else {
                    print!("{}", (output_val as u8) as char);
                }
                i + 2
            }
            5 => {
                let par_1 = self.get(i + 1, arg1_mode);
                let par_2 = self.get(i + 2, arg2_mode);
                if par_1 != 0 {
                    par_2 as usize
                } else {
                    i + 3
                }
            }
            6 => {
                let par_1 = self.get(i + 1, arg1_mode);
                let par_2 = self.get(i + 2, arg2_mode);
                if par_1 == 0 {
                    par_2 as usize
                } else {
                    i + 3
                }
            }
            7 => {
                let val_1 = self.get(i + 1, arg1_mode);
                let val_2 = self.get(i + 2, arg2_mode);
                let res = if val_1 < val_2 { 1 } else { 0 };
                self.set(i + 3, arg3_mode, res);
                i + 4
            }
            8 => {
                let val_1 = self.get(i + 1, arg1_mode);
                let val_2 = self.get(i + 2, arg2_mode);
                let res = if val_1 == val_2 { 1 } else { 0 };
                self.set(i + 3, arg3_mode, res);
                i + 4
            }
            9 => {
                let val_1 = self.get(i + 1, arg1_mode);
                self.base += val_1;
                i + 2
            }
            _ => 1,
        };
        self.ptr = next_i;
        op
    }

    fn run(&mut self) {
        while self.ptr < self.memory.len() {
            let op = self.execute();
            if op == 99 {
                break;
            }
        }
    }
}
