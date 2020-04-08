use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

fn first() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut found = HashSet::new();
    let mut current = input.lines().map(|l| l.trim()).collect::<String>();
    let mut next = String::new();
    while !found.contains(&current.to_owned()) {
        for (i, c) in current.chars().enumerate() {
            let y = i / width;
            let x = i % width;
            let mut neighbors = 0;
            if x > 0 && current.chars().nth(y * width + x - 1).unwrap() == '#' {
                neighbors += 1;
            }
            if x < width - 1 && current.chars().nth(y * width + x + 1).unwrap() == '#' {
                neighbors += 1;
            }
            if y > 0 && current.chars().nth((y - 1) * width + x).unwrap() == '#' {
                neighbors += 1;
            }
            if y < height - 1 && current.chars().nth((y + 1) * width + x).unwrap() == '#' {
                neighbors += 1;
            }
            if (c == '#' && neighbors == 1) || (c == '.' && (neighbors == 1 || neighbors == 2)) {
                next.push('#');
            } else {
                next.push('.');
            }
        }
        found.insert(current.to_owned());
        current = next.to_owned();
        next.clear();
    }
    let mut biodiversity = 0;
    for (i, c) in current.chars().enumerate() {
        if c == '#' {
            biodiversity += 1 << i;
        }
    }
    biodiversity
}

fn second() -> u64 {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut levels = VecDeque::new();
    let empty = ".........................";
    let current = input.lines().map(|l| l.trim()).collect::<String>();
    levels.push_back(current.to_owned());
    for _ in 0..200 {
        levels.push_front(String::from(empty));
        levels.push_back(String::from(empty));
    }
    for _ in 0..200 {
        let mut next_levels = VecDeque::new();
        for (level, current) in levels.iter().enumerate() {
            let mut outer = None;
            if level != 0 {
                outer = Some(levels[level - 1].to_owned());
            }
            let mut inner = None;
            if level != levels.len() - 1 {
                inner = Some(levels[level + 1].to_owned());
            }
            if outer != None && inner != None {
                let outer = outer.unwrap();
                let inner = inner.unwrap();
                let mut next = String::new();
                for (i, c) in current.chars().enumerate() {
                    let y = i / width;
                    let x = i % width;
                    let mut neighbors = 0;
                    if x == 0 && outer.chars().nth(11).unwrap() == '#' {
                        neighbors += 1;
                    }
                    if x > 0 && i != 13 && current.chars().nth(y * width + x - 1).unwrap() == '#' {
                        neighbors += 1;
                    }
                    if x < width - 1
                        && i != 11
                        && current.chars().nth(y * width + x + 1).unwrap() == '#'
                    {
                        neighbors += 1;
                    }
                    if x == width - 1 && outer.chars().nth(13).unwrap() == '#' {
                        neighbors += 1;
                    }
                    if i == 13 {
                        for j in 0..5 {
                            if inner.chars().nth(4 + j * 5).unwrap() == '#' {
                                neighbors += 1;
                            }
                        }
                    }
                    if i == 11 {
                        for j in 0..5 {
                            if inner.chars().nth(j * 5).unwrap() == '#' {
                                neighbors += 1;
                            }
                        }
                    }

                    if y == 0 && outer.chars().nth(7).unwrap() == '#' {
                        neighbors += 1;
                    }
                    if y > 0 && i != 17 && current.chars().nth((y - 1) * width + x).unwrap() == '#'
                    {
                        neighbors += 1;
                    }
                    if y < height - 1
                        && i != 7
                        && current.chars().nth((y + 1) * width + x).unwrap() == '#'
                    {
                        neighbors += 1;
                    }
                    if y == height - 1 && outer.chars().nth(17).unwrap() == '#' {
                        neighbors += 1;
                    }
                    if i == 7 {
                        for j in 0..5 {
                            if inner.chars().nth(j).unwrap() == '#' {
                                neighbors += 1;
                            }
                        }
                    }
                    if i == 17 {
                        for j in 0..5 {
                            if inner.chars().nth(j + 20).unwrap() == '#' {
                                neighbors += 1;
                            }
                        }
                    }

                    if ((c == '#' && neighbors == 1)
                        || (c == '.' && (neighbors == 1 || neighbors == 2)))
                        && (i != 12)
                    {
                        next.push('#');
                    } else {
                        next.push('.');
                    }
                }
                next_levels.push_back(next);
            }
        }
        next_levels.push_front(String::from(empty));
        next_levels.push_back(String::from(empty));
        levels = next_levels;
    }
    levels
        .into_iter()
        .filter(|s| s != &String::from(empty))
        .map(|s| {
            s.chars().fold(0, |acc, c| {
                acc + match c {
                    '#' => 1,
                    _ => 0,
                }
            })
        })
        .sum()
}

fn main() {
    println!("first: {}", first());
    println!("second: {}", second());
}
