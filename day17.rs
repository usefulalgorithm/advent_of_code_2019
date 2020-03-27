use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

struct Robot {
    map: Vec<Vec<char>>,
    current_line: Vec<char>,
    facing: Direction,
    start: (usize, usize),
    end: (usize, usize),
    turns: HashSet<(usize, usize)>,
    input: String,
    dust: i64,
}

impl Robot {
    fn new() -> Self {
        Robot {
            map: Vec::new(),
            current_line: Vec::new(),
            facing: Direction::North,
            start: (0, 0),
            end: (0, 0),
            turns: HashSet::new(),
            input: String::new(),
            dust: 0,
        }
    }
    #[allow(dead_code)]
    fn print_map(&self) {
        let m = &self.map;
        for (y, row) in m.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                print!("{}", m[y][x]);
            }
            println!("");
        }
    }
    fn create_routine(&mut self) -> String {
        let m = &self.map;
        for (y, row) in m.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if is_end(m, y, x) {
                    self.end = (y, x);
                    self.turns.insert((y, x));
                }
                if is_turn(m, y, x) {
                    self.turns.insert((y, x));
                }
            }
        }
        let mut current = self.start;
        let mut patterns_v = Vec::new();
        while current != self.end {
            let turn = rotate(m, current.0, current.1, &mut self.facing);
            let distance = find_next(&mut self.turns, &mut current, self.facing);
            patterns_v.push(Movement::new(turn, distance));
        }

        let patterns_string = patterns_v
            .iter()
            .map(|p| p.stringify() + ",")
            .collect::<String>();

        let mut patterns: HashMap<char, String> = HashMap::new();

        for i in 2..21 {
            patterns.insert('A', (&patterns_string)[0..i].to_string());
            if is_valid_pattern(&patterns[&'A'])
                && find_substring(&patterns_string, &patterns[&'A'])
            {
                let remain = remove_pattern_from(&patterns_string, &patterns[&'A']);
                for j in 2..21 {
                    patterns.insert('B', (&remain)[0..j].to_string());
                    if is_valid_pattern(&patterns[&'B'])
                        && &patterns[&'A'] != &patterns[&'B']
                        && find_substring(&remain, &patterns[&'B'])
                    {
                        let remain = remove_pattern_from(&remain, &patterns[&'B']);
                        for k in 2..21 {
                            if k < remain.len() {
                                patterns.insert('C', (&remain)[0..k].to_string());
                                if is_valid_pattern(&patterns[&'C'])
                                    && &patterns[&'A'] != &patterns[&'C']
                                    && &patterns[&'B'] != &patterns[&'C']
                                    && find_substring(&remain, &patterns[&'C'])
                                {
                                    let remain = remove_pattern_from(&remain, &patterns[&'C']);
                                    if remain.is_empty() {
                                        return parse_routine(&patterns_string, &patterns);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        String::new()
    }
}

fn parse_routine(patterns: &str, m: &HashMap<char, String>) -> String {
    let mut result = String::new();
    let mut head = 0;
    while head < patterns.len() {
        let c = m
            .iter()
            .filter_map(|(k, v)| {
                if &&patterns[head..head + v.len()] == v {
                    Some(k)
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        head += m[c].len();
        result.push(*c);
        result.push(match head < patterns.len() {
            true => ',',
            false => '\n',
        });
    }
    result.push_str(
        &"ABC"
            .chars()
            .map(|c| {
                let mut s = m[&c].to_owned();
                s.pop();
                s.push('\n');
                s
            })
            .collect::<String>()
            .as_str(),
    );
    result.push_str("N\n");
    result
}

fn remove_pattern_from(source: &str, target: &str) -> String {
    let mut result = String::new();
    let mut prev_head = 0;
    let mut i = 0;
    while i < source.len() {
        if source[i..i + 1] == target[0..1]
            && target.len() <= source[i..].len()
            && target == &source[i..i + target.len()]
        {
            result.push_str(&source[prev_head..i]);
            i += target.len();
            prev_head = i;
        } else {
            i += 1;
        }
    }
    if prev_head + target.len() != source.len() {
        result.push_str(&source[prev_head..]);
    }
    result
}

fn is_valid_pattern(s: &str) -> bool {
    for (i, c) in s.chars().enumerate() {
        if i == s.len() - 2 && !c.is_digit(10) {
            return false;
        }
        if i == s.len() - 1 && c != ',' {
            return false;
        }
    }
    true
}

fn find_substring(source: &str, substring: &str) -> bool {
    if source.len() < substring.len() {
        return false;
    }
    for i in 0..source.len() - substring.len() {
        let mut found = true;
        for j in 0..substring.len() {
            let next = source[(i + j)..(i + j + 1)] == substring[j..j + 1];
            found &= next;
            if !found {
                break;
            }
        }
        if found {
            return true;
        }
    }
    false
}

fn rotate(m: &Vec<Vec<char>>, y: usize, x: usize, direction: &mut Direction) -> Turn {
    let turn = match direction {
        Direction::North => {
            if x > 0 && m[y][x - 1] == '#' {
                Turn::Left
            } else {
                Turn::Right
            }
        }
        Direction::South => {
            if x > 0 && m[y][x - 1] == '#' {
                Turn::Right
            } else {
                Turn::Left
            }
        }
        Direction::West => {
            if y > 0 && m[y - 1][x] == '#' {
                Turn::Right
            } else {
                Turn::Left
            }
        }
        Direction::East => {
            if y > 0 && m[y - 1][x] == '#' {
                Turn::Left
            } else {
                Turn::Right
            }
        }
    };
    *direction = direction.turn(&turn);
    turn
}

#[allow(unused_assignments)]
fn find_next(
    turns: &mut HashSet<(usize, usize)>,
    current: &mut (usize, usize),
    direction: Direction,
) -> u64 {
    let mut next = *current;
    let result = match direction {
        Direction::North => {
            next = *turns
                .iter()
                .filter(|(y, x)| x == &current.1 && y < &current.0)
                .max_by(|x, y| x.0.cmp(&y.0))
                .unwrap();
            current.0 - next.0
        }
        Direction::South => {
            next = *turns
                .iter()
                .filter(|(y, x)| x == &current.1 && y > &current.0)
                .min_by(|x, y| x.0.cmp(&y.0))
                .unwrap();
            next.0 - current.0
        }
        Direction::West => {
            next = *turns
                .iter()
                .filter(|(y, x)| y == &current.0 && x < &current.1)
                .max_by(|x, y| x.1.cmp(&y.1))
                .unwrap();
            current.1 - next.1
        }
        Direction::East => {
            next = *turns
                .iter()
                .filter(|(y, x)| y == &current.0 && x > &current.1)
                .min_by(|x, y| x.1.cmp(&y.1))
                .unwrap();
            next.1 - current.1
        }
    };
    *current = turns.take(&next).unwrap();
    result as u64
}

fn is_turn(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    !is_intersection(m, y, x)
        && (is_upper_right(m, y, x)
            || is_bottom_right(m, y, x)
            || is_upper_left(m, y, x)
            || is_bottom_left(m, y, x))
}

fn is_end(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let height = m.len();
    let width = m[0].len();
    x > 0
        && x < width - 2
        && y > 0
        && y < height - 2
        && m[y][x] == '#'
        && [m[y + 1][x], m[y - 1][x], m[y][x + 1], m[y][x - 1]]
            .iter()
            .filter(|&&c| c == '#')
            .count()
            == 1
}

fn is_intersection(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let height = m.len();
    let width = m[0].len();
    x > 0
        && x < width - 2
        && y > 0
        && y < height - 2
        && m[y][x] == '#'
        && m[y + 1][x] == '#'
        && m[y - 1][x] == '#'
        && m[y][x + 1] == '#'
        && m[y][x - 1] == '#'
}

fn is_upper_right(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let height = m.len();
    !is_intersection(m, y, x)
        && y < height - 2
        && x > 0
        && m[y][x] == '#'
        && m[y + 1][x] == '#'
        && m[y][x - 1] == '#'
}

fn is_bottom_right(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    !is_intersection(m, y, x)
        && y > 0
        && x > 0
        && m[y][x] == '#'
        && m[y - 1][x] == '#'
        && m[y][x - 1] == '#'
}

fn is_upper_left(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let height = m.len();
    let width = m[0].len();
    !is_intersection(m, y, x)
        && y < height - 2
        && x < width - 2
        && m[y][x] == '#'
        && m[y + 1][x] == '#'
        && m[y][x + 1] == '#'
}

fn is_bottom_left(m: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    let width = m[0].len();
    !is_intersection(m, y, x)
        && y > 0
        && x < width - 2
        && m[y][x] == '#'
        && m[y - 1][x] == '#'
        && m[y][x + 1] == '#'
}

impl Bus for Robot {
    fn input(&mut self) -> i64 {
        let result = (self.input.chars().next().unwrap() as u8) as i64;
        self.input = self.input.as_str()[1..].to_owned();
        result
    }
    fn output(&mut self, v: i64) {
        let c = (v as u8) as char;
        match c {
            '#' | '.' | '^' | 'v' | '<' | '>' => {
                match c {
                    '^' | '>' | 'v' | '<' => {
                        self.facing = Direction::parse(c);
                        self.start = (self.map.len() as usize, self.current_line.len() as usize);
                    }
                    _ => (),
                };
                self.current_line.push(c);
            }
            '\n' => {
                self.map.push(self.current_line.clone());
                self.current_line.clear();
            }
            _ => self.dust = v,
        };
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn stringify(&self) -> String {
        match *self {
            Turn::Left => "L".to_string(),
            Turn::Right => "R".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn parse(c: char) -> Direction {
        match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("unknown direction: {}", c),
        }
    }
    fn turn(&self, t: &Turn) -> Direction {
        match t {
            Turn::Left => match *self {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
            Turn::Right => match *self {
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
            },
        }
    }
}

type Moves = u64;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Movement {
    turn: Turn,
    moves: Moves,
}

impl Movement {
    fn new(turn: Turn, moves: Moves) -> Self {
        Movement { turn, moves }
    }
    fn stringify(&self) -> String {
        self.turn.stringify() + "," + &self.moves.to_string()
    }
}

fn find_routine() -> String {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let memory = parse(&input);
    let robot = Robot::new();
    let mut intcode = IntCode::new(memory, robot);
    intcode.run();
    intcode.bus.create_routine()
}
fn second() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut memory = parse(&input);
    memory[0] = 2;
    let robot = Robot::new();
    let mut intcode = IntCode::new(memory, robot);
    intcode.bus.input = find_routine();
    intcode.run();
    intcode.bus.dust as u64
}

fn first() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let memory = parse(&input);
    let robot = Robot::new();
    let mut intcode = IntCode::new(memory, robot);
    intcode.run();
    let m = &intcode.bus.map;
    let mut result = 0;
    for (y, row) in m.iter().enumerate() {
        let height = m.len();
        let width = row.len();
        for (x, _) in row.iter().enumerate() {
            if x > 0 && x < width - 2 && y > 0 && y < height - 2 {
                if m[y][x] == '#'
                    && m[y + 1][x] == '#'
                    && m[y - 1][x] == '#'
                    && m[y][x + 1] == '#'
                    && m[y][x - 1] == '#'
                {
                    result += y * x;
                }
            }
        }
    }
    result as u64
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
    fn new(memory: Vec<i64>, bus: T) -> Self {
        IntCode {
            memory,
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
