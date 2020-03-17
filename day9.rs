use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(f);
    let prog: Vec<i128> = reader
        .split(b',')
        .map(|i| {
            let s = String::from_utf8(i.unwrap()).unwrap();
            s.trim().parse::<i128>().unwrap()
        })
        .collect();

    let input = env::args()
        .nth(2)
        .unwrap_or("1".to_string())
        .parse::<i128>()
        .unwrap();
    let input = vec![input];
    let output = run(prog, input);
    println!("{:?}", output);
}

fn run(mut prog: Vec<i128>, inputs: Vec<i128>) -> Vec<i128> {
    let mut output = Vec::new();
    let mut i = 0;
    let mut skip = 2;
    let mut jump = false;
    let mut base = 0;
    let mut cur_input = inputs.iter();

    let double_args = vec![5, 6];
    let triple_args = vec![1, 2, 7, 8];

    while i < prog.len() {
        let opcode = prog[i];
        let op = opcode % 100;
        let modes = opcode / 100;
        if op == 99 {
            break;
        } else if op > 9 || op < 1 {
            panic!("unknown opcode {}", opcode);
        } else {
            let mut input = None;
            if op == 3 {
                input = cur_input.next();
            }
            let (i1, i2, out) = parse_opcode(&prog,
                                             op,
                                             modes,
                                             base,
                                             &double_args,
                                             &triple_args,
                                             i);
            handle_op(
                &mut prog,
                op,
                (i1, i2, out),
                &mut output,
                input,
                &mut jump,
                &mut skip,
                &mut base,
            );
        }
        if jump == true {
            i = skip;
        } else {
            i += skip;
        }
        skip = 2;
        jump = false;
    }
    output
}

#[allow(unused_assignments)]
fn parse_opcode(
    prog: &Vec<i128>,
    op: i128,
    modes: i128,
    base: i128,
    double_args: &Vec<i128>,
    triple_args: &Vec<i128>,
    i: usize,
) -> (i128, i128, i128) {
    let (mut i1, mut i2, mut out) = (-1, -1, -1);
    let (mode1, mode2, mode3) = (modes % 10, (modes / 10) % 10, (modes / 100) % 10);
    i1 = match mode1 {
        0 => prog[i + 1],
        1 => {
            if op == 3 {
                panic!("cannot write in immediate mode");
            } else {
                (i + 1) as i128
            }
        }
        2 => base + prog[i + 1],
        _ => panic!("unknown mode {}", mode1),
    };
    {
        let arg = if double_args.contains(&op) {
            &mut out
        } else {
            &mut i2
        };
        *arg = match mode2 {
            0 => prog[i + 2],
            1 => (i + 2) as i128,
            2 => base + prog[i + 2],
            _ => panic!("unknown mode {}", mode2),
        };
    }
    if triple_args.contains(&op) {
        out = match mode3 {
            0 => prog[i + 3],
            2 => base + prog[i + 3],
            _ => panic!("unknown mode {}", mode3),
        };
    }
    (i1, i2, out)
}

fn handle_op(
    prog: &mut Vec<i128>,
    op: i128,
    (i1, i2, out): (i128, i128, i128),
    output: &mut Vec<i128>,
    input: Option<&i128>,
    jump: &mut bool,
    skip: &mut usize,
    base: &mut i128,
) {
    let (i1, i2, out) = (i1 as usize, i2 as usize, out as usize);
    // boundary check
    for i in vec![i1, i2, out] {
        if i > (prog.len() - 1) && i < usize::max_value() {
            prog.resize_with(i + 1, Default::default);
        }
    }
    match op {
        3 => prog[i1] = *input.unwrap(),
        4 => output.push(prog[i1]),
        9 => *base += prog[i1],
        5 | 6 => {
            *jump = if op == 5 {
                prog[i1] != 0
            } else {
                prog[i1] == 0
            };
            *skip = if *jump == true {
                prog[out] as usize
            } else {
                *skip + 1
            };
        }
        _ => {
            *skip += 2;
            prog[out] = match op {
                1 => prog[i1] + prog[i2],
                2 => prog[i1] * prog[i2],
                7 => {
                    if prog[i1] < prog[i2] {
                        1
                    } else {
                        0
                    }
                }
                8 => {
                    if prog[i1] == prog[i2] {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("unknown op {}", op),
            };
        }
    }
}
