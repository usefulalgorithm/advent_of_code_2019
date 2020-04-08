use crossbeam::thread;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::sync::{Arc, Mutex};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    id: i64,
    x: i64,
    y: i64,
}

impl Instruction {
    fn new(id: i64, x: i64, y: i64) -> Self {
        Self { id, x, y }
    }
}

impl From<&VecDeque<i64>> for Instruction {
    fn from(q: &VecDeque<i64>) -> Self {
        if q.len() != 3 {
            return Self::new(-1, -1, -1);
        }
        Self::new(q[0], q[1], q[2])
    }
}

struct Socket {
    incoming: VecDeque<i64>,
    outgoing: VecDeque<i64>,
    id: i64,
    buffer: Arc<Mutex<VecDeque<Instruction>>>,
}

impl Socket {
    fn new(buffer: Arc<Mutex<VecDeque<Instruction>>>, id: i64) -> Self {
        Self {
            incoming: VecDeque::new(),
            outgoing: VecDeque::new(),
            id,
            buffer,
        }
    }
}

impl Bus for Socket {
    fn input(&mut self) -> i64 {
        if self.incoming.is_empty() {
            let mut buf = self.buffer.lock().unwrap();
            if (*buf).is_empty() || (*buf)[0].id != self.id {
                return -1;
            }
            let inst = (*buf).pop_front().unwrap();
            if inst.x == 99 && inst.y == 99 {
                return 99;
            }
            self.incoming.push_back(inst.x);
            self.incoming.push_back(inst.y);
        }
        let v = self.incoming.pop_front().unwrap();
        v
    }
    fn output(&mut self, v: i64) {
        self.outgoing.push_back(v);

        if let 3 = self.outgoing.len() {
            let mut buf = self.buffer.lock().unwrap();
            let inst = Instruction::from(&self.outgoing);
            if let 255 = inst.id {
                for i in 0..50 {
                    (*buf).push_back(Instruction::new(i, 99, 99));
                }
            }
            (*buf).push_back(inst);
            self.outgoing.clear();
        }
    }
}

struct Network {
    memory: Vec<i64>,
}

impl Network {
    fn new(memory: Vec<i64>) -> Self {
        Self { memory }
    }
    fn run(&mut self) -> i64 {
        let buf = Arc::new(Mutex::new(VecDeque::new()));
        thread::scope(|s| {
            for i in 0..50 {
                let memory = self.memory.clone();
                let buf = Arc::clone(&buf);
                s.spawn(move |_| {
                    let mut c = IntCode::new(&memory, Socket::new(buf, i));
                    c.bus.incoming.push_back(i);
                    c.run();
                });
            }
        })
        .unwrap();
        let mut b = buf.lock().unwrap();
        b.pop_front().unwrap().y
    }
}

fn first() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let memory = parse(&input);
    let mut network = Network::new(memory);
    network.run() as u64
}

struct NatSocket {
    incoming: VecDeque<i64>,
    outgoing: VecDeque<i64>,
    id: i64,
    buffer: Arc<Mutex<VecDeque<Instruction>>>,
}

impl NatSocket {
    fn new(buffer: Arc<Mutex<VecDeque<Instruction>>>, id: i64) -> Self {
        Self {
            incoming: VecDeque::new(),
            outgoing: VecDeque::new(),
            id,
            buffer,
        }
    }
}

impl Bus for NatSocket {
    fn input(&mut self) -> i64 {
        if self.incoming.is_empty() {
            let mut buf = self.buffer.lock().unwrap();
            if (*buf).is_empty() || (*buf)[0].id != self.id {
                return -1;
            }
            let inst = (*buf).pop_front().unwrap();
            if inst.x == 99 && inst.y == 99 {
                return 99;
            }
            self.incoming.push_back(inst.x);
            self.incoming.push_back(inst.y);
        }
        let v = self.incoming.pop_front().unwrap();
        v
    }
    fn output(&mut self, v: i64) {
        self.outgoing.push_back(v);

        if let 3 = self.outgoing.len() {
            let mut buf = self.buffer.lock().unwrap();
            let inst = Instruction::from(&self.outgoing);
            (*buf).push_back(inst);
            self.outgoing.clear();
        }
    }
}

struct NatNetwork {
    memory: Vec<i64>,
}

impl NatNetwork {
    fn new(memory: Vec<i64>) -> Self {
        Self { memory }
    }
    fn run(&mut self) -> i64 {
        let buf = Arc::new(Mutex::new(VecDeque::new()));
        thread::scope(|s| {
            for i in 0..50 {
                let memory = self.memory.clone();
                let buf = Arc::clone(&buf);
                s.spawn(move |_| {
                    let mut c = IntCode::new(&memory, NatSocket::new(buf, i));
                    c.bus.incoming.push_back(i);
                    c.run();
                });
            }
            let buf = Arc::clone(&buf);
            s.spawn(move |_| {
                let mut last_sent = Instruction::new(0, 0, 0);
                loop {
                    let mut inst = Instruction::new(0, 0, 0);
                    let mut buf = buf.lock().unwrap();
                    if !(*buf).is_empty() && (*buf)[0].id == 255 {
                        let incoming = (*buf).pop_front().unwrap();
                        inst.x = incoming.x;
                        inst.y = incoming.y;
                    }
                    if (*buf).is_empty() && inst != Instruction::new(0, 0, 0) {
                        if inst.y == last_sent.y {
                            for i in 0..50 {
                                (*buf).push_back(Instruction::new(i, 99, 99));
                            }
                            (*buf).push_back(inst);
                            break;
                        }
                        (*buf).push_back(inst);
                        last_sent = inst;
                    }
                }
            });
        })
        .unwrap();
        let mut b = buf.lock().unwrap();
        b.pop_front().unwrap().y
    }
}

fn second() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let memory = parse(&input);
    let mut network = NatNetwork::new(memory);
    network.run() as u64
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
