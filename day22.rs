use std::env;
use std::fs;
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Action {
    Cut(i32),
    New,
    Inc(usize),
}

impl Action {
    fn parse(s: &str) -> Action {
        if s.contains("deal into new stack") {
            Action::New
        } else if s.contains("deal with increment ") {
            let n = s.split_at("deal with increment ".len()).1.parse::<usize>().unwrap();
            Action::Inc(n)
        } else if s.contains("cut ") {
            let n = s.split_at("cut ".len()).1.parse::<i32>().unwrap();
            Action::Cut(n)
        } else {
            panic!("Unknown action: {}", s);
        }
    }
}

fn first() -> u128 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut cards = (0..env::args().nth(2).unwrap().parse::<u64>().unwrap()).collect::<Vec<_>>();
    let which = env::args().nth(3).unwrap().parse::<u64>().unwrap();
    for line in input.trim().lines() {
        match Action::parse(line) {
            Action::New => cards = cards.into_iter().rev().collect(),
            Action::Inc(n) => {
                let mut temp: VecDeque<_> = cards.clone().into_iter().collect();
                let mut i = 0;
                while let Some(x) = temp.pop_front() {
                    cards[i] = x;
                    i = (i + n) % cards.len();
                }
            }
            Action::Cut(n) => {
                let at = match n >= 0 {
                    true => n,
                    false => (cards.len() as i32 + n),
                } as usize;
                let mut temp = cards.split_off(at);
                temp.append(&mut cards);
                cards = temp;
            }
        }
    }
    cards.into_iter().enumerate().find(|(_, v)| *v == which).unwrap().0 as u128
}

fn second() -> u128 {
    0
}

fn main() {
    let part = env::args().nth(4).unwrap_or("1".to_string()).parse::<usize>().unwrap();
    let res = match part {
        1 => first(),
        _ => second(),
    };
    println!("part {}: {}", part, res);
}
