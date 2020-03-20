use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn to_tile(id: i128) -> Tile {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("unknown tile id: {}", id),
        }
    }
}

struct Arcade {
    tiles: HashMap<(i128, i128), Tile>,
    current: (i128, i128),
    ready: usize,
    score: i128,
}

impl Arcade {
    fn new() -> Self {
        Arcade {
            tiles: HashMap::new(),
            current: (0i128, 0i128),
            ready: 0,
            score: 0i128,
        }
    }
}

impl Bus for Arcade {
    fn input(&self) -> i128 {
        let paddle = self
            .tiles
            .iter()
            .find_map(|(k, v)| if *v == Tile::Paddle { Some(k.0) } else { None })
            .unwrap();
        let ball = self
            .tiles
            .iter()
            .find_map(|(k, v)| if *v == Tile::Ball { Some(k.0) } else { None })
            .unwrap();
        (ball - paddle).signum() as i128
    }
    fn output(&mut self, v: i128) {
        match self.ready {
            0 => self.current.0 = v,
            1 => self.current.1 = v,
            2 => {
                match self.current {
                    (-1, 0) => self.score = v,
                    _ => {
                        self.tiles.insert(self.current, Tile::to_tile(v));
                    }
                }
                self.current = (0i128, 0i128);
            }
            _ => panic!("error during parsing: {}", self.ready),
        }
        self.ready = (self.ready + 1) % 3;
    }
}

fn parse(input: &str) -> Vec<i128> {
    let mut result = Vec::new();
    for c in input.trim().split(',') {
        result.push(c.parse::<i128>().unwrap());
    }
    result
}

fn second() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut memory = parse(&input);
    memory[0] = 2;
    let arcade = Arcade::new();
    let mut intcode = IntCode::new(memory, arcade);
    intcode.run();
    println!("{}", intcode.bus.score);
}

fn first() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let memory = parse(&input);
    let arcade = Arcade::new();
    let mut intcode = IntCode::new(memory, arcade);
    intcode.run();
    println!(
        "{}",
        intcode
            .bus
            .tiles
            .values()
            .filter(|t| **t == Tile::Block)
            .count()
    );
}

fn main() {
    first();
    second();
}

enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    fn decode(n: i128) -> ParamMode {
        match n {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Unexpected parameter mode: {}", n),
        }
    }
}

fn decode(n: i128) -> (ParamMode, ParamMode, ParamMode, i128) {
    let de = n % 100;
    let c = ParamMode::decode(n / 100 % 10);
    let b = ParamMode::decode(n / 1000 % 10);
    let a = ParamMode::decode(n / 10000 % 10);
    (a, b, c, de)
}

trait Bus {
    fn input(&self) -> i128;
    fn output(&mut self, v: i128);
}

struct IntCode<T>
where
    T: Bus,
{
    memory: Vec<i128>,
    ptr: usize,
    bus: T,
    base: i128,
}

impl<T> IntCode<T>
where
    T: Bus,
{
    fn new(memory: Vec<i128>, bus: T) -> Self {
        IntCode {
            memory,
            ptr: 0,
            bus,
            base: 0,
        }
    }

    fn read(&mut self, i: usize) -> i128 {
        if self.memory.len() <= i {
            self.memory.resize(i + 1, 0);
        }
        self.memory[i]
    }

    fn write(&mut self, i: usize, val: i128) {
        if self.memory.len() <= i {
            self.memory.resize(i + 1, 0);
        }
        self.memory[i] = val;
    }

    fn get(&mut self, i: usize, mode: ParamMode) -> i128 {
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

    fn set(&mut self, i: usize, mode: ParamMode, val: i128) {
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

    fn execute(&mut self) -> i128 {
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
