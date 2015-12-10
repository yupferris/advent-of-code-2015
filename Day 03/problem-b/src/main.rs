use std::io::Read;
use std::fs::File;

use std::collections::HashSet;

type Pos = (i32, i32);

struct Santa {
    pos: Pos
}

impl Santa {
    fn new(houses: &mut HashSet<Pos>) -> Santa {
        let ret = Santa {
            pos: (0, 0)
        };
        ret.visit(houses);
        ret
    }

    fn visit(&self, houses: &mut HashSet<Pos>) {
        houses.insert(self.pos);
    }

    fn move_and_visit(&mut self, c: char, houses: &mut HashSet<Pos>) {
        self.pos = match c {
            '^' => (self.pos.0, self.pos.1 + 1),
            'v' => (self.pos.0, self.pos.1 - 1),
            '<' => (self.pos.0 - 1, self.pos.1),
            '>' => (self.pos.0 + 1, self.pos.1),
            _ => unreachable!()
        };
        self.visit(houses);
    }
}

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let mut houses = HashSet::new();

    let santa_directions = s.chars().enumerate().filter_map(|(i, x)| if (i & 1) == 0 { Some(x) } else { None });
    let robo_santa_directions = s.chars().enumerate().filter_map(|(i, x)| if (i & 1) == 1 { Some(x) } else { None });

    simulate_santa(santa_directions, &mut houses);
    simulate_santa(robo_santa_directions, &mut houses);

    println!("{} house(s) received at least one present! :D", houses.len());
}

fn simulate_santa<I: Iterator<Item = char>>(directions: I, houses: &mut HashSet<Pos>) {
    let mut santa = Santa::new(houses);
    for c in directions {
        santa.move_and_visit(c, houses);
    }
}
