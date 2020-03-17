use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

#[derive(Debug, cmp::PartialEq, cmp::Eq)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn new(x: i32, y: i32) -> Asteroid {
        Asteroid {x, y}
    }
    fn collinear(&self, a: &Asteroid, b: &Asteroid) -> bool {
        let m1 = (self.y - a.y) as f64 / (self.x - a.x) as f64;
        let m2 = (b.y - a.y) as f64 / (b.x - a.x) as f64;
        m1 == m2
    }
    fn between(&self, a: &Asteroid, b: &Asteroid) -> bool {
        let (xa, xb) = (cmp::max(a.x, b.x), cmp::min(a.x, b.x));
        let (ya, yb) = (cmp::max(a.y, b.y), cmp::min(a.y, b.y));
        if (xb..xa+1).contains(&self.x) && (yb..ya+1).contains(&self.y) {
            if xa == xb || self.collinear(&a, &b) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let f = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut asteroids = Vec::new();
    let reader = BufReader::new(f);
    let (mut width, mut height): (i32, i32) = (0, 0);
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push(Asteroid::new(x as i32, y as i32));
            }
        }
        if let 0 = width {
            width = line.len() as i32;
        }
        height += 1;
    }

    // part 1

    let mut a_i = 0;
    let mut res = 0;
    // XXX bad bad bad
    for (v, i) in asteroids.iter().enumerate() {
        let mut temp = 0;
        for j in asteroids.iter() {
            if i != j {
                let mut found = false;
                for k in asteroids.iter() {
                    if j != k && i != k {
                        if k.between(&i, &j) == true {
                            found = true;
                            break;
                        }
                    }
                }
                if found == false {
                    temp += 1;
                }
            }
        }
        if temp > res {
            res = temp;
            a_i = v;
        }
    }
    println!("{:?} can see the most = {}", asteroids[a_i], res);

    // part 2
    let center = &asteroids[a_i];
    let mut counter = 0;
    let mut visited = Vec::new();
    let n = 200;
    loop {
        for x in center.x..width {
            let b = Asteroid::new(x, -1);
            find(&center, &asteroids, b, &mut counter, &mut visited);
            if counter == n {
                break;
            }
        }
        if counter == n {
            break;
        }
        for y in -1..height {
            let b = Asteroid::new(width, y);
            find(&center, &asteroids, b, &mut counter, &mut visited);
            if counter == n {
                break;
            }
        }
        if counter == n {
            break;
        }
        for x in (-1..width+1).rev() {
            let b = Asteroid::new(x, height);
            find(&center, &asteroids, b, &mut counter, &mut visited);
            if counter == n {
                break;
            }
        }
        if counter == n {
            break;
        }
        for y in (0..height).rev() {
            let b = Asteroid::new(-1, y);
            find(&center, &asteroids, b, &mut counter, &mut visited);
            if counter == n {
                break;
            }
        }
        if counter == n {
            break;
        }
        for x in -1..center.x {
            let b = Asteroid::new(x, -1);
            find(&center, &asteroids, b, &mut counter, &mut visited);
            if counter == n {
                break;
            }
        }
        if counter == n {
            break;
        }
    }
    println!("{:?}", asteroids[*visited.last().unwrap()]);
}

fn find(center: &Asteroid,
        asteroids: &Vec<Asteroid>,
        b: Asteroid,
        counter: &mut i32,
        visited: &mut Vec<usize>) {
    let mut a = &b;
    let mut j = 0;
    for (i, c) in asteroids.iter().enumerate() {
        if c != center && c.between(&a, &center) && visited.contains(&i) == false {
            a = c;
            j = i;
        }
    }
    if b != *a {
        *counter += 1;
        println!("destroying ast.no {} : {:?}", *counter, *a);
        visited.push(j);
    }
}
