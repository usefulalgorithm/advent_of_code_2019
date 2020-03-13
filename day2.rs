use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead};

fn run(prog: &mut Vec<u32>) -> u32 {
    for i in 0..(prog.len() / 4) {
        let op_code = prog[4 * i];
        if let 99 = op_code {
            break;
        }
        let (i_1, i_2) = (prog[prog[4*i+1] as usize], prog[prog[4*i+2] as usize]);
        let oi = prog[4*i+3];
        {
            let out = &mut prog[oi as usize];
            *out = match op_code {
                1 => (i_1 + i_2),
                2 => (i_1 * i_2),
                _ => panic!("invalid op code"),
            };
        }
    }
    prog[0]
}

fn main() {
    let f = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(f);
    let golden : Vec<u32> = reader.split(b',')
        .map(|i| {
            let s = String::from_utf8(i.unwrap()).unwrap();
            s.trim().parse::<u32>().unwrap()
        }).collect();

    // part 1
    let mut prog = golden.clone();
    prog[1] = 12;
    prog[2] = 2;
    println!("{}\n", run(&mut prog));

    // part 2
    for i in 0..golden.len() {
        for j in 0..golden.len() {
            let mut prog = golden.clone();
            prog[1] = i as u32;
            prog[2] = j as u32;
            let ret = run(&mut prog);
            if ret == 19690720 {
                println!("{}", 100*i + j);
                break;
            }
        }
    }
}
