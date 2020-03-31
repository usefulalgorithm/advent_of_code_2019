// use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

enum Axis {
    X,
    Y,
}

struct Scanner {
    counter: u64,
    limit: u64,
    axis: Axis,
    output: u64,
}

impl Scanner {
    fn new(counter: u64, limit: u64) -> Self {
        Self {
            counter,
            limit,
            axis: Axis::X,
            output: 0,
        }
    }
}

impl Bus for Scanner {
    #[allow(unused_assignments)]
    fn input(&mut self) -> i64 {
        let mut result = 0;
        match self.axis {
            Axis::X => {
                self.axis = Axis::Y;
                result = self.counter % self.limit;
            }
            Axis::Y => {
                self.axis = Axis::X;
                result = self.counter / self.limit;
            }
        };
        result as i64
    }
    fn output(&mut self, v: i64) {
        self.output = v as u64;
    }
}

fn first() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let memory = parse(&input);
    let mut grid = vec![vec![0; 50]; 50];
    for i in 0..2500 {
        let scanner = Scanner::new(i as u64, 50);
        let mut intcode = IntCode::new(&memory, scanner);
        intcode.run();
        grid[i / 50][i % 50] = intcode.bus.output;
    }
    grid.iter().fold(0, |acc, r| acc + r.iter().sum::<u64>())
}

fn second() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let memory = parse(&input);
    let size = 1000;
    let low = 950;
    let target = 100;

    for y in low..size {
        let mut rightmost = 0;
        for x in 0..size {
            let scanner = Scanner::new((y * size + x) as u64, size as u64);
            let mut intcode = IntCode::new(&memory, scanner);
            intcode.run();
            if let 1 = intcode.bus.output {
                rightmost = x;
            }
        }
        let n_x = rightmost - target + 1;
        let n_y = y + target - 1;
        let scanner = Scanner::new((n_y * size + n_x) as u64, size as u64);
        let mut intcode = IntCode::new(&memory, scanner);
        intcode.run();
        if let 1 = intcode.bus.output {
            return 10000 * n_x + y;
        }
    }
    0
}

fn main() {
    println!("first: {}", first());
    println!("second: {}", second());
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

trait Bus {
    fn input(&mut self) -> i64;
    fn output(&mut self, v: i64);
}

struct IntCode<T>
where
    T: Bus,
{
    memory: Vec<i64>,
    ptr: usize,
    bus: T,
    base: i64,
}

impl<T> IntCode<T>
where
    T: Bus,
{
    fn new(memory: &Vec<i64>, bus: T) -> Self {
        IntCode {
            memory: memory.to_vec(),
            ptr: 0,
            bus,
            base: 0,
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
                let input_val = self.bus.input();
                self.set(i + 1, arg1_mode, input_val);
                i + 2
            }
            4 => {
                let output_val = self.get(i + 1, arg1_mode);
                self.bus.output(output_val);
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
