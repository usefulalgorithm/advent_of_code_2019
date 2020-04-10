use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use std::io;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn parse(s: &String) -> Option<Direction> {
        match s.as_str() {
            "north\n" | "north" => Some(Direction::North),
            "east\n" | "east" => Some(Direction::East),
            "south\n" | "south" => Some(Direction::South),
            "west\n" | "west" => Some(Direction::West),
            _ => None,
        }
    }
    fn opposite(d: Direction) -> Direction {
        match d {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    fn to_string(&self) -> String {
        match self {
            Direction::North => String::from("north"),
            Direction::South => String::from("south"),
            Direction::East => String::from("east"),
            Direction::West => String::from("west"),
        }
    }
    fn to_command(&self) -> String {
        match self {
            Direction::North => String::from("north\n"),
            Direction::South => String::from("south\n"),
            Direction::East => String::from("east\n"),
            Direction::West => String::from("west\n"),
        }
    }
}

fn get_path(
    navigation: &HashMap<String, HashMap<Direction, String>>,
    start: &String,
    end: &String,
) -> Vec<String> {
    let mut queue = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut result = vec![];
    if start == end {
        return result;
    }
    queue.push_back(vec![start.to_owned()]);
    while let Some(cur) = queue.pop_front() {
        let loc = cur[..].last().unwrap();
        if loc == end {
            for (i, l) in cur.iter().enumerate() {
                if i > 0 {
                    for (d, n) in navigation[&cur[i - 1]].iter() {
                        if n == l {
                            result.push(d.to_command());
                        }
                    }
                }
            }
            break;
        }
        visited.insert(loc.to_string());
        for (_, n) in navigation[loc].iter() {
            if !n.is_empty() && !visited.contains(n) {
                let mut next = cur.clone();
                next.push(n.to_string());
                queue.push_back(next);
            }
        }
    }
    result.into_iter().rev().collect()
}

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let memory = parse(&input);
    let mut intcode = IntCode::new(&memory);

    let mut next_moves = vec![];
    let mut next_items = vec![];
    let bad = [
        "molten lava",
        "giant electromagnet",
        "infinite loop",
        "escape pod",
        "photons",
    ];

    let mut navigation = HashMap::new();
    let mut my_items = vec![];
    let mut current_location = String::new();
    let mut previous_location = String::new();
    let mut next_command = String::new();
    let mut pending_commands = vec![];

    loop {
        if !pending_commands.is_empty() {
            println!("pending commands: {:?}", pending_commands);
        }
        if let Some(c) = pending_commands.pop() {
            next_command = c;
        }
        let direction = Direction::parse(&next_command);
        if intcode.input_buf.is_empty() {
            std::mem::swap(&mut intcode.input_buf, &mut next_command);
            intcode.output_buf.clear();
        }
        intcode.run();
        // print to screen!
        print!("{}", intcode.output_buf);
        let lines = intcode
            .output_buf
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let mut locations = vec![];
        for (i, line) in lines.iter().enumerate() {
            if line.contains("==") {
                current_location = (&line[3..line.len() - 3]).to_string();
                navigation
                    .entry(current_location.to_owned())
                    .or_insert(HashMap::new());
                locations.push(current_location.to_owned());
            }
            if line == "Doors here lead:" {
                next_moves.clear();
                let mut j = 1;
                while !lines[i + j].is_empty() {
                    let direction = (&lines[i + j][2..]).to_string();
                    if let Some(n) = navigation.get_mut(&current_location) {
                        (*n).entry(Direction::parse(&direction).unwrap())
                            .or_insert(String::new());
                    }
                    next_moves.push(direction);
                    j += 1;
                }
            }
            if line == "Items here:" {
                next_items.clear();
                let mut j = 1;
                while !lines[i + j].is_empty() {
                    let item = &lines[i + j][2..];
                    if !bad.contains(&item) {
                        next_items.push(item.to_string());
                    }
                    j += 1;
                }
            }
        }
        if let Some(dir) = direction {
            match locations.len() {
                0 => unreachable!(),
                1 => {
                    if let Some(n) = navigation.get_mut(&previous_location) {
                        (*n).insert(dir, current_location.to_owned());
                    }
                    if let Some(n) = navigation.get_mut(&current_location) {
                        (*n).insert(Direction::opposite(dir), previous_location.to_owned());
                    }
                }
                _ => {
                    if let Some(n) = navigation.get_mut(&previous_location) {
                        (*n).insert(dir, locations[0].to_string());
                    }
                    if let Some(n) = navigation.get_mut(&locations[0]) {
                        (*n).insert(Direction::opposite(dir), previous_location.to_owned());
                    }
                }
            }
        }
        if !next_items.is_empty() {
            next_command = String::from("take ");
            let item = &next_items.pop().unwrap();
            my_items.push(item.to_string());
            next_command.push_str(item);
            next_command.push('\n');
        }
        if next_command.is_empty() {
            if let Some(d) = navigation[&current_location]
                .iter()
                .filter_map(|(k, v)| if v.is_empty() { Some(k) } else { None })
                .next()
            {
                next_command = d.to_string();
                next_command.push('\n');
            }
        }
        previous_location = current_location.to_owned();

        println!("\n\nNext Command: {}", next_command);
        if next_command.is_empty() {
            let mut next = None;
            for (c, m) in navigation.iter() {
                for (_, n) in m.iter() {
                    if n.is_empty() {
                        next = Some(c);
                    }
                }
            }
            match next {
                Some(d) => pending_commands = get_path(&navigation, &current_location, &d),
                None => {
                    pending_commands = get_path(
                        &navigation,
                        &current_location,
                        &"Security Checkpoint".to_string(),
                    )
                }
            }
            if pending_commands.is_empty() {
                break;
            }
        }
    }
    // drop everything
    let all_items = my_items.clone();
    drop_items(&mut intcode, &my_items);
    let items_list = (1..all_items.len() + 1)
        .into_iter()
        .flat_map(|i| {
            all_items
                .to_owned()
                .into_iter()
                .combinations(i)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let go = navigation[&"Security Checkpoint".to_string()]
        .iter()
        .find(|(_, v)| *v == &"Pressure-Sensitive Floor".to_string())
        .unwrap()
        .0
        .to_command();
    for items in items_list.into_iter() {
        take_items(&mut intcode, &items);
        let mut cmd = go.to_owned();
        std::mem::swap(&mut intcode.input_buf, &mut cmd);
        intcode.output_buf.clear();
        intcode.run();
        println!("{}", intcode.output_buf);
        if intcode
            .output_buf
            .contains("Analysis complete! You may proceed.")
        {
            println!("Passed with items {:?}", items);
            println!(
                "Password = {}",
                intcode
                    .output_buf
                    .lines()
                    .filter_map(|line| {
                        line.split_whitespace()
                            .filter_map(|word| word.parse::<u64>().ok())
                            .next()
                    })
                    .next()
                    .unwrap()
            );
            return;
        }
        drop_items(&mut intcode, &items);
    }
}

fn take_items(intcode: &mut IntCode, items: &Vec<String>) {
    let mut pending_commands = items
        .into_iter()
        .map(|i| {
            let mut next_command = String::from("take ");
            next_command.push_str(&i);
            next_command.push('\n');
            next_command
        })
        .collect::<Vec<String>>();
    loop {
        if intcode.input_buf.is_empty() {
            match pending_commands.pop() {
                Some(mut cmd) => {
                    std::mem::swap(&mut intcode.input_buf, &mut cmd);
                    intcode.output_buf.clear();
                }
                None => break,
            }
        }
        intcode.run();
        print!("{}", intcode.output_buf);
    }
}

fn drop_items(intcode: &mut IntCode, items: &Vec<String>) {
    let mut pending_commands = items
        .into_iter()
        .map(|i| {
            let mut next_command = String::from("drop ");
            next_command.push_str(&i);
            next_command.push('\n');
            next_command
        })
        .collect::<Vec<String>>();
    loop {
        if intcode.input_buf.is_empty() {
            match pending_commands.pop() {
                Some(mut cmd) => {
                    std::mem::swap(&mut intcode.input_buf, &mut cmd);
                    intcode.output_buf.clear();
                }
                None => break,
            }
        }
        intcode.run();
        print!("{}", intcode.output_buf);
    }
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
    output_buf: String,
}

impl IntCode {
    fn new(memory: &Vec<i64>) -> Self {
        IntCode {
            memory: memory.to_vec(),
            ptr: 0,
            base: 0,
            input_buf: String::new(),
            output_buf: String::new(),
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
        let (arg3_mode, arg2_mode, arg1_mode, mut op) = decode(code);
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
                    match io::stdin().read_line(&mut buffer) {
                        Err(error) => panic!("Error: {}", error),
                        Ok(_) => self.input_buf.push_str(&buffer),
                    }
                }
                let input_val = ((&self.input_buf[0..1]).parse::<char>().unwrap() as u8) as i64;
                self.set(i + 1, arg1_mode, input_val);
                self.input_buf.remove(0);
                i + 2
            }
            4 => {
                let output_val = self.get(i + 1, arg1_mode);
                let output = ((output_val) as u8) as char;
                self.output_buf.push(output);
                if self.output_buf.contains("Command?\n") {
                    op = 99;
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
