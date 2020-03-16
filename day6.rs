use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct Record {
    parent: String,
    children: Vec<String>,
    orbits: u32,
}

impl Record {
    fn new() -> Record {
        Record { parent: String::new(), children: Vec::new(), orbits: 0 }
    }
    fn update_children(&mut self, name: &str) {
        self.children.push(name.to_string());
    }
    fn update_parent(&mut self, name: &str) {
        self.parent = name.to_string();
    }
    fn update_orbits(&mut self, orbits: u32) {
        self.orbits = orbits;
    }
    fn get_children(&self, v: &mut Vec<String>) {
        for i in &self.children {
            v.push(i.to_string());
        }
    }
}

fn main() {
    let f = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(f);
    let mut m: HashMap<String, Record> = HashMap::new();
    // first, stuff all orbit into map
    for line in reader.lines() {
        let v = line.unwrap();
        let v = v.split(')').collect::<Vec<&str>>();
        let (parent, child) = (v[0].to_string(), v[1].to_string());
        {
            let parent = m.entry(parent).or_insert(Record::new());
            parent.update_children(&child);
        }
        let child = m.entry(child).or_insert(Record::new());
        child.update_parent(v[0]);
    }
    
    let mut children = Vec::new();
    {
        m.get("COM").unwrap().get_children(&mut children);
    }
    if !children.is_empty() {
        update_map(&mut m, &children, 1);
    }

    // part 1
    println!("{}", m.iter().fold(0, |acc, (_, r)| acc + r.orbits));

    // part 2
    let mut cur = m.get("YOU");
    let mut my_path = HashSet::new();
    while let Some(r) = cur {
        let parent = &r.parent;
        my_path.insert(parent);
        cur = m.get(parent);
    }
    cur = m.get("SAN");
    while let Some(r) = cur {
        let parent = &r.parent;
        if let Some(_) = my_path.get(parent) {
            break;
        }
        cur = m.get(parent);
    }
    println!("{}", m.get("YOU").unwrap().orbits + m.get("SAN").unwrap().orbits - 2 * cur.unwrap().orbits);
}

fn update_map(m: &mut HashMap<String, Record>, children: &Vec<String>, orbits: u32) {
    for child in children {
        let mut ch = Vec::new();
        {
            let c = m.get_mut(child).unwrap();
            c.get_children(&mut ch);
            c.update_orbits(orbits);
        }

        if ch.len() > 0 {
            update_map(m, &ch, orbits + 1);
        }
    }
}
