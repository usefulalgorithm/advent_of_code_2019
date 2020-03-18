use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use image;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn cw(&self) -> Direction {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn ccw(&self) -> Direction {
        match &self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    Black,
    White,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y, }
    }
    fn walk(&mut self, dir: Direction) {
        match dir {
            Direction::Up       => self.y -= 1,
            Direction::Down     => self.y += 1,
            Direction::Right    => self.x += 1,
            Direction::Left     => self.x -= 1,
        }
    }
}

struct Robot {
    visited: BTreeMap<Coordinate, Color>,
    pos: Coordinate,
    dir: Direction,
    paint_next: bool,
}

impl Robot {
    fn new() -> Self {
        Robot {
            visited: BTreeMap::new(),
            pos: Coordinate::new(0, 0),
            dir: Direction::Up,
            paint_next: true,
        }
    }
    fn detect(&self) -> Color {
        if let Some(v) = self.visited.get(&self.pos) {
            *v
        } else {
            Color::Black
        }
    }
    fn paint(&mut self, color: Color) {
        self.visited.insert(self.pos, color);
    }
    fn walk(&mut self, val: i128) {
        match val {
            0 => {
                self.dir = self.dir.ccw();
                self.pos.walk(self.dir);
            },
            1 => {
                self.dir = self.dir.cw();
                self.pos.walk(self.dir);
            },
            _ => panic!("Unexpected value {}", val),
        }
    }
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

impl Bus for Robot {
    fn input(&self) -> i128 {
        match self.detect() {
            Color::Black => 0,
            Color::White => 1,
        }
    }
    fn output(&mut self, val: i128) {
        if self.paint_next {
            let color = match val {
                0 => Color::Black,
                1 => Color::White,
                _ => panic!("Unknown color: {}", val),
            };
            self.paint(color);
        } else {
            self.walk(val);
        }
        self.paint_next = !self.paint_next;
    }
}

struct IntCode<T> 
where T: Bus {
    memory: Vec<i128>,
    ptr: usize,
    bus: T,
    base: i128,
}

impl<T> IntCode<T>
where T: Bus {
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
            },
            ParamMode::Immediate => self.read(i),
            ParamMode::Relative => {
                let pos = self.read(i) + self.base;
                self.read(pos as usize)
            },
        }
    }

    fn set(&mut self, i: usize, mode: ParamMode, val: i128) {
        match mode {
            ParamMode::Position => {
                let pos = self.read(i) as usize;
                self.write(pos, val);
            },
            ParamMode::Immediate => self.write(i, val),
            ParamMode::Relative => {
                let pos = self.read(i) + self.base;
                self.write(pos as usize, val);
            },
        }
    }

    fn execute(&mut self) -> i128 {
        let i = self.ptr;
        let code = self.memory[i];
        let (arg3_mode, arg2_mode, arg1_mode, op) = decode(code);
        let next_i = match op {
            1 | 2 => {
                let val_1 = self.get(i+1, arg1_mode);
                let val_2 = self.get(i+2, arg2_mode);
                let res = match op {
                    1 => val_1 + val_2,
                    2 => val_1 * val_2,
                    _ => 0,
                };
                self.set(i+3, arg3_mode, res);
                i + 4
            },
            3 => {
                let input_val = self.bus.input();
                self.set(i+1, arg1_mode, input_val);
                i + 2
            },
            4 => {
                let output_val = self.get(i+1, arg1_mode);
                self.bus.output(output_val);
                i + 2
            },
            5 => {
                let par_1 = self.get(i+1, arg1_mode);
                let par_2 = self.get(i+2, arg2_mode);
                if par_1 != 0 {
                    par_2 as usize
                } else {
                    i + 3
                }
            },
            6 => {
                let par_1 = self.get(i+1, arg1_mode);
                let par_2 = self.get(i+2, arg2_mode);
                if par_1 == 0 {
                    par_2 as usize
                } else {
                    i + 3
                }
            },
            7 => {
                let val_1 = self.get(i+1, arg1_mode);
                let val_2 = self.get(i+2, arg2_mode);
                let res = if val_1 < val_2 { 1 } else { 0 };
                self.set(i+3, arg3_mode, res);
                i + 4
            },
            8 => {
                let val_1 = self.get(i+1, arg1_mode);
                let val_2 = self.get(i+2, arg2_mode);
                let res = if val_1 == val_2 { 1 } else { 0 };
                self.set(i+3, arg3_mode, res);
                i + 4
            },
            9 => {
                let val_1 = self.get(i+1, arg1_mode);
                self.base += val_1;
                i + 2
            },
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

fn process(m: &BTreeMap<Coordinate, Color>, name: &str) {
    let mut bottom_right = Coordinate::new(0, 0);
    let mut top_left = Coordinate::new(0, 0);
    for i in m.keys() {
        bottom_right.x = i32::max(bottom_right.x, i.x);
        bottom_right.y = i32::max(bottom_right.y, i.y);
        top_left.x = i32::min(top_left.x, i.x);
        top_left.y = i32::min(top_left.y, i.y);
    }
    let w = (bottom_right.x - top_left.x + 1) as u32;
    let h = (bottom_right.y - top_left.y + 1) as u32;
    let mut img_buf = image::ImageBuffer::new(w, h);
    let black = image::Rgb([0u8, 0u8, 0u8]);
    let white = image::Rgb([255u8, 255u8, 255u8]);
    for (coordinate, color) in m {
        let pixel = match color {
            Color::Black => black,
            Color::White => white,
        };
        let x = (coordinate.x - top_left.x) as u32;
        let y = (coordinate.y - top_left.y) as u32;
        img_buf.put_pixel(x, y, pixel);
    }
    img_buf.save(name).unwrap();
}

fn main() {
    let f = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(f);

    let memory: Vec<i128> = reader
        .split(b',')
        .map(|i| {
            let s = String::from_utf8(i.unwrap()).unwrap();
            s.trim().parse::<i128>().unwrap()
        })
        .collect();

    // part 1
    // let robot = Robot::new();
    // let mut intcode = IntCode::new(memory, robot);
    // intcode.run();
    // println!("{:?}", intcode.bus.visited.len());
    
    // part 2
    let mut robot = Robot::new();
    robot.paint(Color::White);
    let mut intcode = IntCode::new(memory, robot);
    intcode.run();
    process(&intcode.bus.visited, "out.png");
}

