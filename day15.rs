use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Found,
}

impl Tile {
    fn parse(v: i128) -> Tile {
        match v {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Found,
            _ => panic!("Invalid tile type: {}", v),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn from_step(step: (i128, i128)) -> Direction {
        match step {
            (0, 1) => Direction::North,
            (0, -1) => Direction::South,
            (-1, 0) => Direction::West,
            (1, 0) => Direction::East,
            _ => panic!("Invalid step: {:?}", step),
        }
    }
    fn to_step(&self) -> (i128, i128) {
        match *self {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
    fn to_command(&self) -> i128 {
        match *self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i128,
    y: i128,
}

impl Coordinate {
    fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
    fn step(&self, step: (i128, i128)) -> Self {
        Coordinate::new(self.x + step.0, self.y + step.1)
    }
    fn neighbors(&self) -> Vec<Self> {
        vec![
            self.step(Direction::North.to_step()),
            self.step(Direction::South.to_step()),
            self.step(Direction::West.to_step()),
            self.step(Direction::East.to_step()),
        ]
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn get_step(current: &Coordinate, next: &Coordinate) -> (i128, i128) {
    let result = (next.x - current.x, next.y - current.y);
    match result {
        (0, 1) | (0, -1) | (1, 0) | (-1, 0) => result,
        _ => panic!("unknown step {:?}", result),
    }
}

struct Droid2 {
    current: Coordinate,
    next: Option<Coordinate>,
    direction: Direction,
    map: HashMap<Coordinate, (Tile, Cost)>,
    oxygen_system: Option<Coordinate>,
}

impl Droid2 {
    fn new() -> Self {
        let current = Coordinate::new(0, 0);
        let next = Coordinate::new(0, 1);
        let mut map = HashMap::new();
        map.insert(current, (Tile::Empty, 0));

        Droid2 {
            current: current,
            next: Some(next),
            direction: Direction::from_step(get_step(&current, &next)),
            map: map,
            oxygen_system: None,
        }
    }
    fn get_cost(&self, c: &Coordinate) -> Option<u64> {
        let e = self.map.get(c)?;
        Some(e.1)
    }
    fn get_tile(&self, c: &Coordinate) -> Option<Tile> {
        let e = self.map.get(c)?;
        Some(e.0)
    }
    fn get_next(&mut self) -> Option<Coordinate> {
        let c = self
            .current
            .neighbors()
            .into_iter()
            .filter(|c| !self.map.contains_key(c))
            .next();
        if let Some(_) = c {
            return c;
        }
        let result = self
            .current
            .neighbors()
            .into_iter()
            .filter(|c| self.map.get(c).unwrap().0 != Tile::Wall)
            .min_by(|x, y| self.get_cost(&x).unwrap().cmp(&self.get_cost(&y).unwrap()));

        if let Some(r) = result {
            if self.get_cost(&r).unwrap() == 0 {
                return None;
            }
            for n in self.current.neighbors().iter() {
                if *n != r {
                    let v = self.map.get_mut(n).unwrap();
                    if (*v).0 != Tile::Wall {
                        (*v).1 = 1_000_000_000;
                    }
                }
            }
        }
        result
    }
    fn get_time(&self) -> Option<u64> {
        let os = self.oxygen_system?;
        let mut result = 0;
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut frontier: Vec<Coordinate> = os
            .neighbors()
            .into_iter()
            .filter(|i| self.get_tile(i) == Some(Tile::Empty))
            .collect();
        visited.insert(os);

        while !frontier.is_empty() {
            let mut new_frontier = Vec::new();
            for c in frontier.into_iter() {
                visited.insert(c);
                let mut neighbors = c
                    .neighbors()
                    .into_iter()
                    .filter(|i| self.get_tile(i) == Some(Tile::Empty) && !visited.contains(i))
                    .collect();
                new_frontier.append(&mut neighbors);
            }
            frontier = new_frontier;
            result += 1;
        }
        Some(result)
    }
}

impl Bus for Droid2 {
    fn input(&self) -> i128 {
        if let None = self.next {
            return 99;
        }
        self.direction.to_command()
    }
    fn output(&mut self, v: i128) {
        let tile = Tile::parse(v);
        let new_coord = self.current.step(self.direction.to_step());
        let mut cost = self.map.get(&self.current).unwrap().1 + 1;
        match tile {
            Tile::Found => {
                self.oxygen_system = Some(new_coord);

                self.current = new_coord;
                self.next = self.get_next();
                if let Some(n) = self.next {
                    self.direction = Direction::from_step(get_step(&self.current, &n));
                }
            }
            Tile::Empty => {
                self.current = new_coord;
                self.next = self.get_next();
                if let Some(n) = self.next {
                    self.direction = Direction::from_step(get_step(&self.current, &n));
                }
            }
            Tile::Wall => {
                self.next = self.get_next();
                if let Some(n) = self.next {
                    self.direction = Direction::from_step(get_step(&self.current, &n));
                }
                cost = 1_000_000_000;
            }
        }
        if !self.map.contains_key(&new_coord) {
            self.map.insert(new_coord, (tile, cost));
        }
    }
}

type Cost = u64;

struct Droid1 {
    current: Coordinate,
    next: Coordinate,
    direction: Direction,
    map: HashMap<Coordinate, (Tile, Cost)>,
    oxygen_system: Option<Coordinate>,
}

impl Droid1 {
    fn new() -> Self {
        let current = Coordinate::new(0, 0);
        let next = Coordinate::new(0, 1);
        let mut map = HashMap::new();
        map.insert(current, (Tile::Empty, 0));

        Droid1 {
            current: current,
            next: next,
            direction: Direction::from_step(get_step(&current, &next)),
            map: map,
            oxygen_system: None,
        }
    }
    fn get_cost(&self, c: &Coordinate) -> Option<u64> {
        let e = self.map.get(c)?;
        Some(e.1)
    }
    fn get_next(&mut self) -> Coordinate {
        let c = self
            .current
            .neighbors()
            .into_iter()
            .filter(|c| !self.map.contains_key(c))
            .next();
        if let Some(result) = c {
            return result;
        }
        let result = self
            .current
            .neighbors()
            .into_iter()
            .filter(|c| self.map.get(c).unwrap().0 == Tile::Empty)
            .min_by(|x, y| self.get_cost(&x).unwrap().cmp(&self.get_cost(&y).unwrap()))
            .unwrap();
        for n in self.current.neighbors().iter() {
            if *n != result {
                let v = self.map.get_mut(n).unwrap();
                if (*v).0 == Tile::Empty {
                    (*v).1 = 1_000_000_000;
                }
            }
        }
        result
    }
}

impl Bus for Droid1 {
    fn input(&self) -> i128 {
        if let Some(_) = self.oxygen_system {
            return 99;
        }
        self.direction.to_command()
    }
    fn output(&mut self, v: i128) {
        let tile = Tile::parse(v);
        let new_coord = self.current.step(self.direction.to_step());
        let mut cost = self.map.get(&self.current).unwrap().1 + 1;
        match tile {
            Tile::Found => {
                self.oxygen_system = Some(new_coord);
            }
            Tile::Empty => {
                self.current = new_coord;
                self.next = self.get_next();
                self.direction = Direction::from_step(get_step(&self.current, &self.next));
            }
            Tile::Wall => {
                self.next = self.get_next();
                self.direction = Direction::from_step(get_step(&self.current, &self.next));
                cost = 1_000_000_000;
            }
        }
        if !self.map.contains_key(&new_coord) {
            self.map.insert(new_coord, (tile, cost));
        }
    }
}

#[allow(dead_code)]
fn draw(map: &HashMap<Coordinate, (Tile, Cost)>) {
    let mut canvas = [[' '; 50]; 50];
    let c = 25;
    for (k, v) in map {
        let x = k.x + c;
        let y = c - k.y;
        canvas[y as usize][x as usize] = match v.0 {
            Tile::Wall => 'X',
            Tile::Empty => 'o',
            Tile::Found => '@',
        };
    }
    for row in canvas.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!("");
    }
}

fn second() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let memory = parse(&input);
    let droid = Droid2::new();
    let mut intcode = IntCode::new(memory, droid);
    intcode.run();
    intcode.bus.get_time().unwrap()
}

fn first() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let memory = parse(&input);
    let droid = Droid1::new();
    let mut intcode = IntCode::new(memory, droid);
    intcode.run();
    intcode
        .bus
        .map
        .iter()
        .filter_map(|(_, v)| {
            if let Tile::Found = v.0 {
                Some(v.1)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn parse(input: &str) -> Vec<i128> {
    let mut result = Vec::new();
    for c in input.trim().split(',') {
        result.push(c.parse::<i128>().unwrap());
    }
    result
}

fn main() {
    println!("first: {}", first());
    println!("second: {}", second());
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
                if let 99 = input_val {
                    return input_val;
                }
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
