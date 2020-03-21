use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

fn parse(input: &str) -> HashMap<&str, (u64, Vec<(u64, &str)>)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" => ");
            let reactants = split
                .next()
                .unwrap()
                .split(", ")
                .map(|x| parse_chemical(x))
                .collect();
            let (product_quantity, product_name) = parse_chemical(split.next().unwrap());
            (product_name, (product_quantity, reactants))
        })
        .collect()
}

fn parse_chemical(input: &str) -> (u64, &str) {
    let mut p = input.split(' ');
    let q = p.next().unwrap().parse().unwrap();
    let n = p.next().unwrap();
    (q, n)
}

fn make_fuel<'a>(
    fuel_quantity: u64,
    reactions: &HashMap<&str, (u64, Vec<(u64, &'a str)>)>,
    extras: &mut HashMap<&'a str, u64>,
) -> u64 {
    let mut deque = VecDeque::new();
    deque.push_back(("FUEL", fuel_quantity));
    let mut result = 0;
    loop {
        match deque.pop_front() {
            Some(("ORE", mut ore_quantity)) => {
                let extra_ore = extras.entry("ORE").or_insert(0);
                let extra_used = std::cmp::min(*extra_ore, ore_quantity);
                ore_quantity -= extra_used;
                *extra_ore -= extra_used;
                result += ore_quantity;
            }
            Some((product, mut quantity)) => {
                let extra = extras.entry(product).or_insert(0);
                let extra_used = std::cmp::min(*extra, quantity);
                quantity -= extra_used;
                *extra -= extra_used;
                if quantity > 0 {
                    let (product_quantity, reactants) = reactions.get(product).unwrap();
                    let multiplier = (quantity - 1) / product_quantity + 1;
                    *extra = product_quantity * multiplier - quantity;
                    for (reactant_quantity, reactant_name) in reactants {
                        deque.push_back((reactant_name, reactant_quantity * multiplier));
                    }
                }
            }
            None => {
                return result;
            }
        }
    }
}

fn part2(input: &str) {
    let reactions = parse(input);
    let mut extras = HashMap::new();
    let ores_per_fuel = make_fuel(1, &reactions, &mut extras);
    let mut result = 1;
    if extras.contains_key("FUEL") {
        result += extras.remove("FUEL").unwrap();
    }
    let produced_fuel_multiplier = result;
    *extras.entry("ORE").or_insert(0) = 1_000_000_000_000 - ores_per_fuel;
    loop {
        let producible = std::cmp::max(1, extras.get("ORE").unwrap() / ores_per_fuel * produced_fuel_multiplier);
        let needed_ores = make_fuel(producible, &reactions, &mut extras);
        if needed_ores != 0 {
            println!("{}", result);
            return;
        }
        result += producible;
    }
}

fn part1(input: &str) {
    println!("{}", make_fuel(1, &parse(input), &mut HashMap::new()));
}

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    part1(&input);
    part2(&input);
}
