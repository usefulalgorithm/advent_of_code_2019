use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use std::cmp;

fn main() {
    let f = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(f);
    let prog: Vec<i32> = reader.split(b',')
        .map(|i| {
            let s = String::from_utf8(i.unwrap()).unwrap();
            s.trim().parse::<i32>().unwrap()
        }).collect();

    let inputs = (5..10).permutations(5);
    let mut res = 0;
    for phases in inputs {
        let mut temp = 0;
        let mut amps = vec![prog.clone(); 5];
        let mut states = vec![0; 5];
        let mut n = 0;
        let mut phases = phases.iter().rev().collect::<Vec<_>>();
        loop {
            println!("amp.{}", n);
            let mut input = Vec::<i32>::new();
            if let Some(p) = phases.pop() {
                input.push(*p);
            }
            input.push(temp);
            match run(&mut amps[n], input, &mut states[n as usize]) {
                Some(r) => {
                    temp = r;
                },
                None => {
                    if amps[n][states[n]] == 99 {
                        println!("amp.{} halted", n);
                        if n == 4 {
                            break;
                        }
                    } else {
                        println!("amp.{} waiting for input", n);
                    }
                },
            };
            n = (n+1) % 5;
        }
        res = cmp::max(res, temp);
    }
    println!("{}", res);
}

fn run(prog: &mut Vec<i32>, input: Vec<i32>, i: &mut usize) -> Option<i32> {
    let mut skip = 2;
    let mut jump = false;
    let mut cur_in = input.iter();
    while *i < prog.len() {
        println!("what is prog[{}]? {}", i, prog[*i]);
        match prog[*i] % 100 {
            3 => {
                let index = prog[*i+1];
                prog[index as usize] = *cur_in.next()?;
            },
            4 => {
                let index = if prog[*i] < 100 { prog[*i+1] as usize } else { *i+1 };
                *i += skip;
                return Some(prog[index]);
            },
            5 | 6 => {
                let mode1 = (prog[*i] / 100) % 10;
                let mode2 = prog[*i] / 1000;
                let ip = match mode1 {
                    1 => prog[*i+1],
                    0 => prog[prog[*i+1] as usize],
                    _ => panic!("unknown mode {}", mode1),
                };
                let out_i = match mode2 {
                    1 => *i+2,
                    0 => prog[*i+2] as usize,
                    _ => panic!("unknown mode {}", mode2),
                };
                let op = prog[out_i] as usize;
                jump = if prog[*i] % 100 == 5 && ip != 0 {
                    true
                } else if prog[*i] % 100 == 6 && ip == 0 {
                    true
                } else {
                    false
                };
                skip = if jump == false {
                    skip + 1
                } else {
                    op
                };
            },
            99 => break,
            _ => {
                handle_op(prog, *i);
                skip += 2;
            },
        }
        if jump {
            *i = skip;
        } else {
            *i += skip;
        }
        skip = 2;
        jump = false;
    }
    None
}

fn handle_op(prog: &mut Vec<i32>, index: usize) {
    let code = format!("{:04}", prog[index]);
    let op = &code[2..];
    let mut it = code.chars();
    let mode2 = it.next().unwrap();
    let mode1 = it.next().unwrap();
    let c1 = prog[index+1];
    let c2 = prog[index+2];
    let out = prog[index+3] as usize;
    let num1 = match mode1 {
        '0' => prog[c1 as usize],
        '1' => c1,
        _ => panic!("unknown mode"),
    };
    let num2 = match mode2 {
        '0' => prog[c2 as usize],
        '1' => c2,
        _ => panic!("unknown mode"),
    };
    prog[out] = match op {
        "01" => num1 + num2,
        "02" => num1 * num2,
        "07" => { if num1 < num2 { 1 } else { 0 } }, // less than
        "08" => { if num1 == num2 { 1 } else { 0 } }, // equal
        _ => panic!("unknown op: {}", op),
    };
}
