use std::collections::VecDeque;
use std::env;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Action {
    Cut(i128),
    New,
    Inc(u128),
}

impl Action {
    fn parse(s: &str) -> Action {
        if s.contains("deal into new stack") {
            Action::New
        } else if s.contains("deal with increment ") {
            let n = s
                .split_at("deal with increment ".len())
                .1
                .parse::<u128>()
                .unwrap();
            Action::Inc(n)
        } else if s.contains("cut ") {
            let n = s.split_at("cut ".len()).1.parse::<i128>().unwrap();
            Action::Cut(n)
        } else {
            panic!("Unknown action: {}", s);
        }
    }
}

fn first() -> u128 {
    let input = fs::read_to_string(env::args().nth(2).unwrap()).unwrap();
    let mut cards = (0..env::args().nth(3).unwrap().parse::<u64>().unwrap()).collect::<Vec<_>>();
    let which = env::args().nth(4).unwrap().parse::<u64>().unwrap();
    for line in input.trim().lines() {
        match Action::parse(line) {
            Action::New => cards = cards.into_iter().rev().collect(),
            Action::Inc(n) => {
                let mut temp: VecDeque<_> = cards.clone().into_iter().collect();
                let mut i = 0;
                while let Some(x) = temp.pop_front() {
                    cards[i] = x;
                    i = (i + n as usize) % cards.len();
                }
            }
            Action::Cut(n) => {
                let at = match n >= 0 {
                    true => n,
                    false => (cards.len() as i128 + n),
                } as usize;
                let mut temp = cards.split_off(at);
                temp.append(&mut cards);
                cards = temp;
            }
        }
    }
    cards
        .into_iter()
        .enumerate()
        .find(|(_, v)| *v == which)
        .unwrap()
        .0 as u128
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn second() -> u128 {
    let cards: i128 = 119315717514047;
    let target = 2020;
    let times: u128 = 101741582076661;
    let input = fs::read_to_string(env::args().nth(2).unwrap()).unwrap();
    let (mut offset_diff, mut increment_mul) = (0i128, 1i128);
    let inv = |i| mod_pow(i, cards as u128 - 2, cards as u128) as i128;
    for line in input.trim().lines() {
        match Action::parse(line) {
            Action::New => {
                increment_mul *= -1;
                increment_mul %= cards;
                offset_diff += increment_mul;
                offset_diff %= cards;
            }
            Action::Inc(n) => {
                increment_mul *= inv(n);
                increment_mul %= cards;
            }
            Action::Cut(n) => {
                offset_diff += increment_mul * n as i128;
                offset_diff %= cards;
            }
        }
    }
    increment_mul += cards;
    offset_diff += cards;
    let increment = mod_pow(increment_mul as u128, times, cards as u128);
    let mut offset = offset_diff * (1 - increment as i128) % cards;
    offset *= inv((((1 - increment_mul) % cards) + cards) as u128) % cards;
    offset %= cards;
    offset += cards;
    (offset as u128+ target * increment) % cards as u128
}

fn main() {
    let part = env::args()
        .nth(1)
        .unwrap_or("1".to_string())
        .parse::<usize>()
        .unwrap();
    let res = match part {
        1 => first(),
        _ => second(),
    };
    println!("part {}: {}", part, res);
}
