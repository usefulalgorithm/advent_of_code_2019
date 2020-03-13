use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter;
use std::env;

fn main() {
    let filename = env::args()
        .nth(1)
        .unwrap().to_string();
    // part 1
    let f = File::open(&filename).unwrap();
    let reader = BufReader::new(f);

    let mut res = 0;
    for line in reader.lines() {
        let mut num : u32 = line.unwrap().parse().unwrap();
        res += num / 3 - 2;
    }
    println!("{}", res);

    // part 2
    let f = File::open(&filename).unwrap();
    let reader = BufReader::new(f);
    let mut res: u128 = 0;
    for line in reader.lines() {
        let mut num: u128 = line.unwrap().parse().unwrap();
        res += iter::from_fn(move || {
            if num > 5 {
                num = num / 3 - 2;
                Some(num)
            }
            else {
                None
            }
        }).sum::<u128>();
    }
    println!("\n{}", res);
}
