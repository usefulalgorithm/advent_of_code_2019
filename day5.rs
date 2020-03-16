use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(f);
    let prog: Vec<i32> = reader.split(b',')
        .map(|i| {
            let s = String::from_utf8(i.unwrap()).unwrap();
            s.trim().parse::<i32>().unwrap()
        }).collect();

    let input = env::args().nth(2).unwrap_or("1".to_string()).parse::<i32>().unwrap();
    let input = vec![input];
    let output = run(prog, input);
    println!("{:?}", output);
}

fn run(mut prog: Vec<i32>, input: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    let mut i = 0;
    let mut skip = 2;
    let mut jump = false;
    while i < prog.len() {
        match prog[i] % 100 {
            3 => {
                let index = prog[i+1];
                prog[index as usize] = input[0];
            },
            4 => {
                let index = if prog[i] < 100 { prog[i+1] as usize } else { i+1 };
                output.push(prog[index]);
            },
            5 | 6 => {
                let mode1 = (prog[i] / 100) % 10;
                let mode2 = prog[i] / 1000;
                let ip = match mode1 {
                    1 => prog[i+1],
                    0 => prog[prog[i+1] as usize],
                    _ => panic!("unknown mode {}", mode1),
                };
                let out_i = match mode2 {
                    1 => i+2,
                    0 => prog[i+2] as usize,
                    _ => panic!("unknown mode {}", mode2),
                };
                let op = prog[out_i] as usize;
                jump = if prog[i] % 100 == 5 && ip != 0 {
                    true
                } else if prog[i] % 100 == 6 && ip == 0 {
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
                handle_op(&mut prog, i);
                skip += 2;
            },
        }
        if jump {
            i = skip;
        } else {
            i += skip;
        }
        skip = 2;
        jump = false;
    }
    output
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
        "05" => 0, // jump if true
        "06" => 0, // jump if false
        "07" => { if num1 < num2 { 1 } else { 0 } }, // less than
        "08" => { if num1 == num2 { 1 } else { 0 } }, // equal
        _ => panic!("unknown op: {}", op),
    };
}
