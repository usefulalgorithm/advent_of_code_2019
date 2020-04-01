use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Portal(u64),
}

impl Tile {
    fn get_portal_name(&self) -> Option<String> {
        match self {
            Tile::Portal(n) => {
                let mut result = String::new();
                for i in 0..26 {
                    if n & (1 << i) != 0 {
                        result.push(('A' as u8 + i) as char);
                    }
                }
                Some(result)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    y: i32,
    x: i32,
}

impl Coordinate {
    fn new(y: i32, x: i32) -> Self {
        Self { y, x }
    }
    fn neighbors(&self) -> [Self; 4] {
        [
            Self {
                y: self.y - 1,
                x: self.x,
            },
            Self {
                y: self.y + 1,
                x: self.x,
            },
            Self {
                y: self.y,
                x: self.x - 1,
            },
            Self {
                y: self.y,
                x: self.x + 1,
            },
        ]
    }

    fn is_outer(&self, width: u64, height: u64) -> bool {
        self.x == width as i32 - 3 || self.x == 2 || self.y == height as i32 - 2 || self.y == 2
    }
}

fn make_portals(grid: &mut HashMap<Coordinate, Tile>, portal_parts: HashMap<Coordinate, char>) {
    for (coord, c) in &portal_parts {
        match coord
            .neighbors()
            .iter()
            .find(|n| grid.get(n) == Some(&Tile::Empty))
        {
            None => (),
            Some(n) => {
                let d = portal_parts[coord
                    .neighbors()
                    .iter()
                    .find(|n| portal_parts.get(n) != None)
                    .unwrap()];
                let e = grid.entry(*n).or_insert(Tile::Wall);
                *e = Tile::Portal(1 << (*c as u8 - 'A' as u8) | 1 << (d as u8 - 'A' as u8));
            }
        }
    }
}

#[allow(unused_assignments)]
fn parse_input() -> (HashMap<Coordinate, Tile>, u64, u64) {
    let mut result = HashMap::new();
    let mut portal_parts = HashMap::new();
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            result.insert(
                Coordinate::new(y as i32, x as i32),
                match c {
                    ' ' | '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    _ => {
                        portal_parts.insert(Coordinate::new(y as i32, x as i32), c);
                        Tile::Wall
                    }
                },
            );
        }
        height = y;
    }
    make_portals(&mut result, portal_parts);
    (result, width as u64, height as u64)
}

fn gen_paths(grid: &HashMap<Coordinate, Tile>) -> HashMap<String, HashMap<String, u64>> {
    let mut paths = HashMap::new();
    let portals = grid
        .iter()
        .filter(|(_, v)| v.get_portal_name() != None)
        .map(|(k, v)| (*k, *v))
        .collect::<HashMap<Coordinate, Tile>>();
    for (k, v) in portals {
        let mut dists = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited: HashSet<Coordinate> = HashSet::new();
        visited.insert(k);
        queue.push_back((k, 0));
        while let Some((node, steps)) = queue.pop_front() {
            for &neighbor in node.neighbors().iter() {
                if grid.contains_key(&neighbor) && grid.get(&neighbor) != Some(&Tile::Wall) {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        match grid[&neighbor] {
                            Tile::Portal(_) => {
                                dists.insert(grid[&neighbor].get_portal_name().unwrap(), steps + 1);
                            }
                            Tile::Empty => queue.push_back((neighbor, steps + 1)),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        let e = paths
            .entry(v.get_portal_name().unwrap())
            .or_insert(HashMap::new());
        for (k, v) in dists.iter() {
            (*e).insert(k.to_owned(), *v);
        }
    }
    paths
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    s: String,
    c: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.c.cmp(&self.c).then_with(|| self.s.cmp(&other.s))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn first() -> u64 {
    let (grid, _, _) = parse_input();
    let paths = gen_paths(&grid);
    let start = State {
        s: "A".to_string(),
        c: 0,
    };
    let mut dist: HashMap<_, _> = paths.keys().map(|k| (k, u64::max_value())).collect();
    let mut heap = BinaryHeap::new();
    *dist.get_mut(&start.s).unwrap() = 0;
    heap.push(start);

    while let Some(State { s, c }) = heap.pop() {
        if s == "Z".to_string() {
            return c - 1;
        }
        if c > dist[&s] {
            continue;
        }
        for (p, l) in paths[&s].iter() {
            let next = State {
                s: p.to_string(),
                c: l + c + 1,
            };
            if next.c < dist[&p] {
                *dist.get_mut(&p).unwrap() = next.c;
                heap.push(next);
            }
        }
    }
    0
}

fn gen_paths_with_coord(
    grid: &HashMap<Coordinate, Tile>,
) -> HashMap<(String, Coordinate), HashMap<String, (u64, Coordinate)>> {
    let mut paths = HashMap::new();
    let portals = grid
        .iter()
        .filter(|(_, v)| v.get_portal_name() != None)
        .map(|(k, v)| (*k, *v))
        .collect::<HashMap<Coordinate, Tile>>();
    for (k, v) in portals {
        let mut dists = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited: HashSet<Coordinate> = HashSet::new();
        visited.insert(k);
        queue.push_back((k, 0));
        while let Some((node, steps)) = queue.pop_front() {
            for &neighbor in node.neighbors().iter() {
                if grid.contains_key(&neighbor) && grid.get(&neighbor) != Some(&Tile::Wall) {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        match grid[&neighbor] {
                            Tile::Portal(_) => {
                                dists.insert(
                                    grid[&neighbor].get_portal_name().unwrap(),
                                    (steps + 1, neighbor),
                                );
                            }
                            Tile::Empty => queue.push_back((neighbor, steps + 1)),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        let e = paths
            .entry((v.get_portal_name().unwrap(), k))
            .or_insert(HashMap::new());
        for (k, v) in dists.iter() {
            (*e).insert(k.to_owned(), *v);
        }
    }
    paths
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct LevelState {
    name: String,
    cost: u64,
    level: u64,
    coord: Coordinate,
}

impl Ord for LevelState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.level.cmp(&self.level))
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for LevelState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn second() -> u64 {
    let (grid, width, height) = parse_input();
    let paths = gen_paths_with_coord(&grid);
    let portals: HashMap<Coordinate, String> = grid
        .iter()
        .filter_map(|(k, v)| {
            if v.get_portal_name() != None {
                Some((*k, v.get_portal_name().unwrap()))
            } else {
                None
            }
        })
        .collect();

    let start_coord = portals
        .iter()
        .find(|(_, v)| **v == "A".to_string())
        .unwrap()
        .0
        .to_owned();

    let start = LevelState {
        name: "A".to_string(),
        cost: 0,
        level: 0,
        coord: start_coord,
    };
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(start);

    while let Some(LevelState {
        name,
        cost,
        level,
        coord,
    }) = heap.pop()
    {
        if name == "Z".to_string() && level == 0 {
            return cost - 1;
        }
        for (next_name, (next_cost, this_coord)) in paths[&(name.to_owned(), coord)].iter() {
            if level == 0 && this_coord.is_outer(width, height) && next_name != &"A".to_string() && next_name != &"Z".to_string() {
                continue;
            }
            if level != 0 && next_name == &"Z".to_string() {
                continue;
            }
            if next_name == &"A".to_string() {
                continue;
            }
            let next_coord = match next_name == &"Z".to_string() {
                false => {
                    portals
                        .iter()
                        .find(|(c, n)| *n == next_name && *c != this_coord)
                        .unwrap()
                        .0
                        .to_owned()
                }
                true => *this_coord,
            };
            let next = LevelState {
                name: next_name.to_string(),
                cost: next_cost + cost + 1,
                level: match next_coord.is_outer(width, height) {
                    true => level + match next_name == &"Z".to_string() {
                        true => 0,
                        false => 1,
                    },
                    false => level - 1,
                },
                coord: next_coord,
            };
            if !visited.contains(&((coord, next.coord), level)) {
                visited.insert(((next.coord, coord), level));
                heap.push(next);
            }
        }
    }

    0
}

fn main() {
    println!("first: {}", first());
    println!("second: {}", second());
}
