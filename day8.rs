use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut args = env::args();
    let mut f = File::open(args.nth(1).unwrap())?;
    let w = args.next().unwrap_or("25".to_string()).parse::<usize>().unwrap();
    let h = args.next().unwrap_or("6".to_string()).parse::<usize>().unwrap();
    let mut v = Vec::new();
    f.read_to_end(&mut v)?;
    v.pop();
    let pic : Vec<u32> = v.iter()
        .map(|b| char::from(*b).to_digit(10).unwrap())
        .collect();
    let mut layers = Vec::new();
    let length = w * h;
    for i in 0..pic.len() / length {
        layers.push(&pic[i*length..(i+1)*length]);
    }

    // part 1
    /*
    {
        layers.sort_by(|v1, v2| v1.iter()
                      .filter(|&x| *x == 0)
                      .count()
                      .cmp(
                          &v2.iter().filter(|&x| *x == 0)
                          .count()
                          )
                      );
        let (a, b) = (&layers[0].iter().filter(|&x| *x == 1).count(),
                      &layers[0].iter().filter(|&x| *x == 2).count());
        println!("{} * {} = {}", a, b, a*b);
    }
    */

    // part 2
    let mut picture = vec![2; length];
    for layer in layers.iter().rev() {
        for (i, p) in layer.iter().enumerate() {
            if *p != 2 {
                picture[i] = *p;
            }
        }
    }
    display(&picture, w, h);
    Ok(())
}

fn display(picture: &Vec<u32>, width: usize, height: usize) {
    for h in 0..height {
        let s: String = picture[width*h..width*(h+1)]
            .iter()
            .map(|x| match x {
                1 => '*',
                0 => ' ',
                _ => panic!("unknown symbol"),
            })
            .collect();
        println!("{:?}", s);
    }
}
