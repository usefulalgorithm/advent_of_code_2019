use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::ops;
use std::hash::{ Hash, Hasher };

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x.abs();
    let mut y = y.abs();
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

#[derive(Debug, cmp::PartialEq, cmp::Eq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn new(x: i32, y: i32) -> Asteroid {
        Asteroid {x, y}
    }
}

impl fmt::Display for Asteroid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Eq)]
struct Fraction {
    num: i32,
    den: i32,
}

impl Fraction {
    fn new(num: i32, den: i32) -> Self {
        let gcd = gcd(num, den);
        Fraction {
            num: num / gcd,
            den: den / gcd,
        }
    }
}

impl Hash for Fraction {
    fn hash<H:Hasher>(&self, state: &mut H) {
        (self.num.cmp(&0)).hash(state);
        (self.den.cmp(&0)).hash(state);
        self.num.abs().hash(state);
        self.den.abs().hash(state);
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.num.cmp(&0) == other.num.cmp(&0) &&
            self.den.cmp(&0) == other.den.cmp(&0) &&
            self.num.abs() == other.num.abs() &&
            self.den.abs() == other.den.abs()
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}/{})", self.num, self.den)
    }
}

fn slope(a: &Asteroid, b: &Asteroid) -> Fraction {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    if dx == 0 {
        Fraction::new(match dy.cmp(&0) {
            Ordering::Less => i32::min_value() + 1,
            Ordering::Greater => i32::max_value() - 1,
            _ => panic!("{} == {}", a, b),
        }, 1)
    } else {
        Fraction::new(dy, dx)
    }
}

fn main() {
    let f = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut asteroids = Vec::new();
    let reader = BufReader::new(f);
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push(Asteroid::new(x as i32, y as i32));
            }
        }
    }

    // part 1
    let mut m: HashMap<Fraction, Vec<usize>> = HashMap::new();
    let mut index = 0;
    for (i,v) in asteroids.iter().enumerate() {
        let mut mi: HashMap<Fraction, Vec<usize>> = HashMap::new();
        for (j, u) in asteroids.iter().enumerate() {
            if v != u {
                let s = slope(v, u);
                match mi.get_mut(&s) {
                    Some(t) => (*t).push(j),
                    None => {
                        mi.insert(s, vec![j]); 
                    }
                }
            }
        }
        if m.len() < mi.len() {
            m = mi;
            index = i;
        }
    }
    println!("{}: {}", asteroids[index], m.len());
}
