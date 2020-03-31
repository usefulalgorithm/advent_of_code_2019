use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    y: u64,
    x: u64,
}

impl Coordinate {
    fn new(y: u64, x: u64) -> Self {
        Self { y, x }
    }

    fn neighbors(&self) -> [Coordinate; 4] {
        [
            Coordinate::new(self.y - 1, self.x),
            Coordinate::new(self.y + 1, self.x),
            Coordinate::new(self.y, self.x - 1),
            Coordinate::new(self.y, self.x + 1),
        ]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Node(char),
}

fn get_path_to_keys(
    pos: Coordinate,
    graph: &HashMap<Coordinate, Tile>,
) -> HashMap<char, (u64, u64)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut result = HashMap::new();
    queue.push_back((pos, 0, 0));
    visited.insert(pos);
    while let Some((current, steps, mut doors)) = queue.pop_front() {
        for n in current.neighbors().iter().filter(|n| graph.contains_key(n)) {
            if !visited.contains(n) {
                visited.insert(*n);
                match graph[n] {
                    Tile::Wall => {}
                    Tile::Empty => queue.push_back((*n, steps + 1, doors)),
                    Tile::Node(c) => {
                        if c.is_ascii_lowercase() {
                            result.insert(c, (steps + 1, doors));
                        } else if c.is_ascii_uppercase() {
                            doors |= 1 << (c.to_ascii_lowercase() as u8 - 'a' as u8);
                        }
                        queue.push_back((*n, steps + 1, doors));
                    }
                }
            }
        }
    }
    result
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MultiState {
    states: [State; 4],
}

impl MultiState {
    fn new(states: [State; 4]) -> Self {
        Self { states }
    }
    fn cost(&self) -> u64 {
        self.states.iter().fold(0, |acc, s| s.cost + acc)
    }
    fn owned_keys(&self) -> u64 {
        self.states.iter().fold(0, |acc, s| s.owned_keys | acc)
    }
    fn characters(&self) -> [char; 4] {
        self.states
            .iter()
            .map(|s| s.character)
            .collect::<Vec<char>>()[..]
            .try_into()
            .unwrap()
    }
    fn update(&mut self, i: usize, s: State) {
        self.states[i] = s;
    }
}

impl Ord for MultiState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost()
            .cmp(&self.cost())
            .then_with(|| self.owned_keys().cmp(&other.owned_keys()))
            .then_with(|| self.states[0].character.cmp(&other.states[0].character))
            .then_with(|| self.states[1].character.cmp(&other.states[1].character))
            .then_with(|| self.states[2].character.cmp(&other.states[2].character))
            .then_with(|| self.states[3].character.cmp(&other.states[3].character))
    }
}

impl PartialOrd for MultiState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct State {
    cost: u64,
    character: char,
    owned_keys: u64,
}

impl State {
    fn new(cost: u64, character: char, owned_keys: u64) -> Self {
        Self {
            cost,
            character,
            owned_keys,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.owned_keys.cmp(&other.owned_keys))
            .then_with(|| self.character.cmp(&other.character))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse() -> (
    HashMap<Coordinate, Tile>,
    HashMap<char, Coordinate>,
    Coordinate,
) {
    let mut graph = HashMap::new();
    let mut keys = HashMap::new();
    let mut start = Coordinate::new(0, 0);

    for (y, line) in fs::read_to_string(env::args().nth(1).unwrap())
        .unwrap()
        .trim()
        .lines()
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            let coord = Coordinate::new(y as u64, x as u64);
            graph.insert(
                coord,
                match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    _ => {
                        if c.is_ascii_lowercase() {
                            keys.insert(c, coord);
                        } else if !c.is_ascii_uppercase() {
                            start = coord;
                        }
                        Tile::Node(c)
                    }
                },
            );
        }
    }
    (graph, keys, start)
}

fn dijkstra_single(paths_to_keys: &HashMap<char, HashMap<char, (u64, u64)>>, target: usize) -> u64 {
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(State::new(0, '@', 0));
    let mut found = HashSet::new();
    while let Some(current) = priority_queue.pop() {
        if current.owned_keys == (1 << target) - 1 {
            return current.cost;
        }
        if found.contains(&(current.character, current.owned_keys)) {
            continue;
        }
        found.insert((current.character, current.owned_keys));
        for (next_key, (next_cost, need_keys)) in paths_to_keys.get(&current.character).unwrap() {
            let offset = *next_key as u8 - 'a' as u8;
            if need_keys & !current.owned_keys == 0 && current.owned_keys & (1 << offset) == 0 {
                priority_queue.push(State::new(
                    current.cost + next_cost,
                    *next_key,
                    current.owned_keys | (1 << offset),
                ));
            }
        }
    }
    0
}

#[allow(dead_code)]
fn first() -> u64 {
    let (graph, keys, start) = parse();
    let mut paths_to_keys = HashMap::new();
    for (k, coord) in &keys {
        paths_to_keys.insert(*k, get_path_to_keys(*coord, &graph));
    }
    paths_to_keys.insert('@', get_path_to_keys(start, &graph));
    dijkstra_single(&paths_to_keys, keys.len())
}

fn dijkstra_multiple(
    paths_to_keys: &HashMap<char, HashMap<char, (u64, u64)>>,
    target: usize,
) -> u64 {
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(MultiState::new([
        State::new(0, '1', 0),
        State::new(0, '2', 0),
        State::new(0, '3', 0),
        State::new(0, '4', 0),
    ]));
    let mut found = HashSet::new();
    while let Some(current) = priority_queue.pop() {
        if current.owned_keys() == (1 << target) - 1 {
            return current.cost();
        }
        if found.contains(&(current.characters(), current.owned_keys())) {
            continue;
        }
        found.insert((current.characters(), current.owned_keys()));

        for (i, c) in current.characters().iter().enumerate() {
            for (next_key, (next_cost, need_keys)) in paths_to_keys.get(c).unwrap() {
                let offset = *next_key as u8 - 'a' as u8;
                if need_keys & !current.owned_keys() == 0
                    && current.owned_keys() & (1 << offset) == 0
                {
                    let mut next = current;
                    next.update(
                        i,
                        State::new(
                            current.states[i].cost + next_cost,
                            *next_key,
                            current.states[i].owned_keys | (1 << offset),
                        ),
                    );
                    priority_queue.push(next);
                }
            }
        }
    }
    0
}

fn insert_robots(graph: &mut HashMap<Coordinate, Tile>, start: Coordinate) {
    *graph
        .get_mut(&Coordinate::new(start.y - 1, start.x - 1))
        .unwrap() = Tile::Node('1');
    *graph
        .get_mut(&Coordinate::new(start.y - 1, start.x))
        .unwrap() = Tile::Wall;
    *graph
        .get_mut(&Coordinate::new(start.y - 1, start.x + 1))
        .unwrap() = Tile::Node('2');
    *graph
        .get_mut(&Coordinate::new(start.y, start.x - 1))
        .unwrap() = Tile::Wall;
    *graph.get_mut(&Coordinate::new(start.y, start.x)).unwrap() = Tile::Wall;
    *graph
        .get_mut(&Coordinate::new(start.y, start.x + 1))
        .unwrap() = Tile::Wall;
    *graph
        .get_mut(&Coordinate::new(start.y + 1, start.x - 1))
        .unwrap() = Tile::Node('3');
    *graph
        .get_mut(&Coordinate::new(start.y + 1, start.x))
        .unwrap() = Tile::Wall;
    *graph
        .get_mut(&Coordinate::new(start.y + 1, start.x + 1))
        .unwrap() = Tile::Node('4');
}

fn second() -> u64 {
    let (mut graph, keys, start) = parse();
    insert_robots(&mut graph, start);
    let robots = vec![
        ('1', Coordinate::new(start.y - 1, start.x - 1)),
        ('2', Coordinate::new(start.y - 1, start.x + 1)),
        ('3', Coordinate::new(start.y + 1, start.x - 1)),
        ('4', Coordinate::new(start.y + 1, start.x + 1)),
    ];
    let mut paths_to_keys = HashMap::new();
    for (k, coord) in &keys {
        paths_to_keys.insert(*k, get_path_to_keys(*coord, &graph));
    }
    for (c, coord) in robots.iter() {
        paths_to_keys.insert(*c, get_path_to_keys(*coord, &graph));
    }
    println!("{:?}", paths_to_keys);
    dijkstra_multiple(&paths_to_keys, keys.len())
}

fn main() {
    // println!("first: {}", first());
    println!("second: {}", second());
}
