extern crate num;

use std::io::prelude::*;
use std::fs::File;
use num::complex::*;


fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let origin = Complex::new(0, 0);
    let origin_dir = Complex::new(0, 1);
    let left = Complex::new(0, 1);
    let right = Complex::new(0, -1);

    let result = s.split(",").map(|s| s.trim())
        .fold((origin, origin_dir), |(pos, dir), instr| {
            let (turn, dist) = match instr.split_at(1) {
                (hd, tl) => 
                    (match hd {
                        "L" => left,
                        "R" => right,
                        _ => panic!()
                     },
                     tl.parse::<i32>().unwrap()),
            };
            (pos + (dir * turn).scale(dist), dir * turn)
        });

    println!("result: {}", result.0.re.abs() + result.0.im.abs());
}
