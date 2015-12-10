use std::io::Read;
use std::fs::File;

use std::collections::HashSet;

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let mut houses = HashSet::new();
    houses.insert((0, 0));
    s.chars().fold((0, 0), |pos, x| {
        let new_pos = match x {
            '^' => (pos.0, pos.1 + 1),
            'v' => (pos.0, pos.1 - 1),
            '<' => (pos.0 - 1, pos.1),
            '>' => (pos.0 + 1, pos.1),
            _ => unreachable!()
        };
        houses.insert(new_pos);
        new_pos
    });

    println!("{} house(s) received at least one present! :D", houses.len());
}
