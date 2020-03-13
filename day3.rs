use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Segment {
    id: u8,
    start: (i32, i32),
    end: (i32, i32),
    length: i32,
}

fn between(s: i32, e: i32, x: i32) -> bool {
    (x >= s && x <= e) || (x <= s && x >= e)
}

impl Segment {
    fn new(id: u8, start: (i32, i32), end: (i32, i32), length: i32) -> Segment {
        Segment { id, start, end, length, }
    }
    fn is_perpendicular(&self) -> bool {
        self.start.0 == self.end.0
    }
    fn is_horizontal(&self) -> bool {
        !self.is_perpendicular()
    }
    fn is_parallel(&self, x: &Segment) -> bool {
        (self.is_horizontal() && x.is_horizontal()) ||
            (self.is_perpendicular() && x.is_perpendicular())
    }
    fn find_intersect(&self, x: &Segment) -> Option<(i32, i32)> {
        if self.start == (0, 0) && x.start == (0, 0) || self.is_parallel(x) {
            None
        } else {
            if self.is_perpendicular() {
                if between(x.start.0, x.end.0, self.start.0) && between(self.start.1, self.end.1, x.start.1) {
                    Some((self.start.0, x.start.1))
                }
                else {
                    None
                }
            }
            else {
                if between(x.start.1, x.end.1, self.start.1) && between(self.start.0, self.end.0, x.start.0) {
                    Some((x.start.0, self.start.1))
                }
                else {
                    None
                }
            }
        }
    }
}

fn main() {
    let f = File::open(env::args()
                      .nth(1)
                      .unwrap())
        .unwrap();
    let reader = BufReader::new(f);
    let golden : Vec<String> = reader.lines()
        .map(|l| l.unwrap())
        .collect();

    // part 1
    let mut id = 0;
    let mut map : HashMap<u8, Vec<Segment>> = HashMap::new();
    let layout = golden.clone();
    for wires in layout {
        let mut start = (0,0);
        let mut segments = Vec::<Segment>::new();
        let wire = wires.trim().split(',').collect::<Vec<&str>>();
        for segment in wire {
            let dir = &segment[0..1];
            let length = segment[1..].parse::<i32>().unwrap();
            let end = match dir {
                "R" => (start.0, start.1 + length),
                "L" => (start.0, start.1 - length),
                "U" => (start.0 + length, start.1),
                "D" => (start.0 - length, start.1),
                _ => panic!("illegal direction"),
            };
            segments.push(Segment::new(id, start, end, 0));
            start = end;
        }
        map.insert(id, segments);
        id += 1;
    }
    
    let mut res = (0, 0);
    for (i, (_, s_i)) in map.iter().enumerate() {
        for (j, (_, s_j)) in map.iter().enumerate() {
            if i < j {
                for i in s_i {
                    for j in s_j {
                        res = match i.find_intersect(&j) {
                            None => res,
                            Some(x) => {
                                if res == (0,0) || x.0.abs() + x.1.abs() < i32::abs(res.0) + i32::abs(res.1) {
                                    x
                                } else {
                                    res
                                }
                            },
                        }
                    }
                }
            }
        }
    }
    println!("{}", res.0 + res.1);

    // part 2
    let mut id = 0;
    let mut segments = Vec::<Segment>::new();
    let layout = golden.clone();
    for wires in layout {
        let mut start = (0,0);
        let mut cur_len = 0;
        let wire = wires.trim().split(',').collect::<Vec<&str>>();
        for segment in wire {
            let dir = &segment[0..1];
            let length = segment[1..].parse::<i32>().unwrap();
            let end = match dir {
                "R" => (start.0, start.1 + length),
                "L" => (start.0, start.1 - length),
                "U" => (start.0 + length, start.1),
                "D" => (start.0 - length, start.1),
                _ => panic!("illegal direction"),
            };
            segments.push(Segment::new(id, start, end, cur_len));
            start = end;
            cur_len += length;
        }
        id += 1;
    }

    let mut res = i32::max_value();
    for i in 0..segments.len() {
        for j in i..segments.len() {
            if segments[i].id != segments[j].id {
                res = match segments[i].find_intersect(&segments[j]) {
                    None => res,
                    Some(x) => {
                        let dist = distance(&x, &segments[i], &segments[j]);
                        if res > dist {
                            dist
                        } else {
                            res
                        }
                    },
                }
            }
        }
    }
    println!("{}", res);
}

fn distance(p: &(i32, i32), a: &Segment, b: &Segment) -> i32 {
    a.length + b.length + if a.is_perpendicular() {
        i32::abs(b.start.0 - p.0) + i32::abs(a.start.1 - p.1)
    } else {
        i32::abs(a.start.0 - p.0) + i32::abs(b.start.1 - p.1)
    }
}
