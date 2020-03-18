use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

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

#[derive(Debug, PartialEq, Eq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn new(x: i32, y: i32) -> Asteroid {
        Asteroid { x, y }
    }
    fn distance(&self, other: &Self) -> f64 {
        ((other.x - self.x).pow(2) as f64 + (other.y - self.y).pow(2) as f64).sqrt()
    }
    fn value(&self) -> i32 {
        100 * self.x + self.y
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

impl fmt::Display for Quadrant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Quadrant::First => "1",
            Quadrant::Second => "2",
            Quadrant::Third => "3",
            Quadrant::Fourth => "4",
        };
        write!(f, "{}", s)
    }
}

impl Fraction {
    fn new(num: i32, den: i32) -> Self {
        let gcd = gcd(num, den);
        Fraction {
            num: num / gcd,
            den: den / gcd,
        }
    }
    fn quadrant(&self) -> Quadrant {
        match (self.num.cmp(&0), self.den.cmp(&0)) {
            (Ordering::Greater, Ordering::Greater) => Quadrant::First,
            (Ordering::Equal, Ordering::Greater) => Quadrant::First,
            (Ordering::Less, Ordering::Greater) => Quadrant::Second,
            (Ordering::Less, Ordering::Less) => Quadrant::Third,
            (Ordering::Equal, Ordering::Less) => Quadrant::Third,
            (Ordering::Greater, Ordering::Less) => Quadrant::Fourth,
            (_, Ordering::Equal) => panic!("{} equals infinity!", self),
        }
    }
}

impl Hash for Fraction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.num.cmp(&0)).hash(state);
        (self.den.cmp(&0)).hash(state);
        self.num.abs().hash(state);
        self.den.abs().hash(state);
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.num.cmp(&0) == other.num.cmp(&0)
            && self.den.cmp(&0) == other.den.cmp(&0)
            && self.num.abs() == other.num.abs()
            && self.den.abs() == other.den.abs()
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}/{}) [{}]", self.num, self.den, self.quadrant())
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.quadrant() != other.quadrant() {
            self.quadrant().cmp(&other.quadrant())
        } else {
            (self.den as i64 * other.num as i64).cmp(&(self.num as i64 * other.den as i64))
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn slope(a: &Asteroid, b: &Asteroid) -> Fraction {
    let dx = b.x - a.x;
    let dy = a.y - b.y;
    if dx == 0 {
        Fraction::new(
            match dy.cmp(&0) {
                Ordering::Less => i32::min_value() + 1,
                Ordering::Greater => i32::max_value() - 1,
                _ => panic!("{} == {}", a, b),
            },
            1,
        )
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
    for (i, v) in asteroids.iter().enumerate() {
        let mut mi: HashMap<Fraction, Vec<usize>> = HashMap::new();
        for (j, u) in asteroids.iter().enumerate() {
            if v != u {
                let s = slope(v, u);
                let e = mi.entry(s).or_insert(Vec::new());
                e.push(j);
            }
        }
        if m.len() < mi.len() {
            m = mi;
            index = i;
        }
    }
    println!("{}: {}", asteroids[index], m.len());

    // part 2
    for (_, v) in m.iter_mut() {
        (*v).sort_by(|a, b| {
            let da = asteroids[index].distance(&asteroids[*a]);
            let db = asteroids[index].distance(&asteroids[*b]);
            db.partial_cmp(&da).unwrap()
        });
    }
    let mut v = m.into_iter().collect::<Vec<(Fraction, Vec<usize>)>>();
    v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut n = 0;
    loop {
        for i in v.iter_mut() {
            if !(*i).1.is_empty() {
                let a = (*i).1.pop();
                n += 1;
                if n == 200 {
                    println!("{}", asteroids[a.unwrap()].value());
                    break;
                }
            }
        }
        if n == 200 {
            break;
        }
    }
}
