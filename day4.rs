use std::env;
use std::collections::HashMap;

fn main() {
    let (lo, hi) = (env::args().nth(1).unwrap().parse::<u32>().unwrap(),
                    env::args().nth(2).unwrap().parse::<u32>().unwrap());
    let mut res = 0;
    for n in lo..(hi+1) {
        // if has_adjacent_digits(&n) && monotonically_ascending(&n) {
        if check(&n) {
            res += 1;
        }
    }
    println!("{}", res);
}

// part 1
fn has_adjacent_digits(n: &u32) -> bool {
    let mut x = 'x';
    for c in n.to_string().chars() {
        if x != 'x' && x == c {
            return true;
        }
        x = c;
    }
    false
}

fn monotonically_ascending(n: &u32) -> bool {
    let mut x = 'x';
    for c in n.to_string().chars() {
        if x != 'x' && x > c {
            return false;
        }
        x = c;
    }
    true
}

// part 2
fn check(n: &u32) -> bool {
    if !monotonically_ascending(n) {
        return false;
    }
    let mut m = HashMap::new();
    for c in n.to_string().chars() {
        *m.entry(c).or_insert(0) += 1;
    }
    m.retain(|_, &mut v| v == 2);
    m.len() > 0
}
