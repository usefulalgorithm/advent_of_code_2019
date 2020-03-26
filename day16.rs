use std::env;
use std::fs;

fn parse_input() -> Vec<i64> {
    fs::read_to_string(env::args().nth(1).unwrap())
        .unwrap()
        .trim()
        .chars()
        .map(|c| i64::from(c.to_digit(10).unwrap()))
        .collect()
}

fn generate_sieve(n: usize, length: usize) -> Vec<i64> {
    let base_pattern: Vec<i64> = vec![0, 1, 0, -1];
    base_pattern
        .iter()
        .map(|&c| std::iter::repeat(c).take(n).collect::<Vec<i64>>())
        .flatten()
        .cycle()
        .take(length + 1)
        .skip(1)
        .collect::<Vec<i64>>()
}

fn second() {
    let input = parse_input();
    let offset = input[0..7]
        .iter()
        .map(|&c| std::char::from_digit(c as u32, 10).unwrap())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let mut numbers = input
        .iter()
        .cycle()
        .take(input.len() * 10_000)
        .map(|c| *c)
        .collect::<Vec<i64>>();

    for _ in 0..100 {
        let mut partial_sum: i64 = numbers[offset..].iter().sum();
        for i in offset..numbers.len() {
            let temp = partial_sum;
            partial_sum -= numbers[i];
            if temp >= 0 {
                numbers[i] = temp % 10;
            } else {
                numbers[i] = temp.abs() % 10;
            }
        }
    }

    println!(
        "second: {:?}",
        &numbers[offset..offset+8]
            .iter()
            .map(|&c| std::char::from_digit(c as u32, 10).unwrap())
            .collect::<String>()
    );
}

fn first() {
    let numbers = parse_input();
    let sieves: Vec<Vec<i64>> = (1..numbers.len() + 1)
        .map(|i| generate_sieve(i, numbers.len()))
        .collect();

    let mut current = numbers.clone();
    let phases = 100;
    for _ in 0..phases {
        let mut next = current.clone();
        for (i, s) in sieves.iter().enumerate() {
            next[i] = s
                .iter()
                .zip(current.iter())
                .map(|(x, y)| x * y)
                .sum::<i64>()
                .abs()
                % 10;
        }
        current = next;
    }
    println!(
        "first: {:?}",
        &current[0..8]
            .iter()
            .map(|&c| std::char::from_digit(c as u32, 10).unwrap())
            .collect::<String>()
    );
}

fn main() {
    first();
    second();
}
