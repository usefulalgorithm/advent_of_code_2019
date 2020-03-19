use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::cmp::Ordering;
use regex::Regex;
use num::Integer;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z, }
    }
    fn sum(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(pos: Vec3, vel: Vec3) -> Self {
        Self { pos, vel, }
    }
    fn update_velocity(&mut self, other: &Self) {
        self.vel.x += compare(self.pos.x, other.pos.x);
        self.vel.y += compare(self.pos.y, other.pos.y);
        self.vel.z += compare(self.pos.z, other.pos.z);
    }
    fn walk(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
    fn energy(&self) -> i32 {
        self.pos.sum() * self.vel.sum()
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}
fn compare(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Greater => -1,
        Ordering::Equal => 0,
        Ordering::Less => 1,
    }
}

fn read_moon(input: &str) -> Moon {
    let moon_regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let caps = moon_regex.captures(input).unwrap();
    Moon::new(Vec3::new(caps[1].parse().unwrap()
                        , caps[2].parse().unwrap()
                        , caps[3].parse().unwrap())
              , Vec3::new(0, 0, 0))
}

fn step_velocities(pos: &[i32]) -> Vec<i32> {
    let mut result = vec![0; pos.len()];
    for (i1, p1) in pos.iter().enumerate() {
        for (i2, p2) in pos.iter().enumerate().skip(i1+1) {
            result[i1] += compare(*p1, *p2);
            result[i2] += compare(*p2, *p1);
        }
    }
    result
}

fn repeat_axis(mut pos: Vec<i32>) -> u64 {
    let mut velocities = vec![0i32; pos.len()];
    let end_velocities = velocities.clone();
    let mut steps = 0;
    loop {
        let velocity_changes = step_velocities(&pos);
        for (v, c) in velocities.iter_mut().zip(velocity_changes) {
            *v += c;
        }
        for (p, v) in pos.iter_mut().zip(velocities.iter()) {
            *p += v;
        }
        steps += 1;
        if velocities == end_velocities {
            break;
        }
    }
    steps * 2
}

fn main() {
    let f = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(f);
    let mut moons = Vec::new();
    for line in reader.lines() {
        moons.push(read_moon(&line.unwrap()));
    }

    // part 1
    // let n = env::args().nth(2).unwrap_or(10.to_string()).parse::<i32>().unwrap();
    // for _ in 0..n {
        // for i in 0..moons.len() {
        //     for j in (i+1)..moons.len() {
        //         let m1 = moons[i];
        //         let m2 = moons[j];
        //         moons[i].update_velocity(&m2);
        //         moons[j].update_velocity(&m1);
        //     }
        // }
        // for cur in moons.iter_mut() {
        //     cur.walk();
        // }
    // }
    // let energy = moons.iter().fold(0, |acc, m| acc + m.energy());
    // println!("after {} steps, energy = {}", n, energy);

    // part 2
    let res = 1.lcm(&repeat_axis(moons.iter().map(|m| m.pos.x).collect()))
        .lcm(&repeat_axis(moons.iter().map(|m| m.pos.y).collect()))
        .lcm(&repeat_axis(moons.iter().map(|m| m.pos.z).collect()));
    println!("{}", res);
}
