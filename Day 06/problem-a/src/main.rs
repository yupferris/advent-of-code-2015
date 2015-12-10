#![feature(slice_patterns, convert, box_syntax)]

use std::io::Read;
use std::fs::File;

type Pos = (i32, i32);

struct Range {
    start: Pos,
    end: Pos
}

impl Range {
    fn parse(start: &str, end: &str) -> Range {
        Range {
            start: Range::parse_pos(start),
            end: Range::parse_pos(end)
        }
    }

    fn parse_pos(s: &str) -> Pos {
        let parts = s.split(',').collect::<Vec<_>>();
        (Range::parse_comp(parts[0]), Range::parse_comp(parts[1]))
    }

    fn parse_comp(s: &str) -> i32 {
        s.parse::<_>().unwrap()
    }
}

enum Instruction {
    TurnOn(Range),
    TurnOff(Range),
    Toggle(Range)
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        let parts = s.split(' ').collect::<Vec<_>>();
        match parts.as_slice() {
            ["turn", "on", start, "through", end] => Instruction::TurnOn(Range::parse(start, end)),
            ["turn", "off", start, "through", end] => Instruction::TurnOff(Range::parse(start, end)),
            ["toggle", start, "through", end] => Instruction::Toggle(Range::parse(start, end)),
            _ => unreachable!()
        }
    }
}

const LIGHTS_SIZE: usize = 1000;
const TOTAL_LIGHTS: usize = LIGHTS_SIZE * LIGHTS_SIZE;

struct Lights {
    lights: Box<[bool; TOTAL_LIGHTS]>
}

impl Lights {
    fn new() -> Lights {
        Lights {
            lights: box [false; TOTAL_LIGHTS]
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            &Instruction::TurnOn(ref range) => self.apply_range(range, |light| { *light = true; }),
            &Instruction::TurnOff(ref range) => self.apply_range(range, |light| { *light = false; }),
            &Instruction::Toggle(ref range) => self.apply_range(range, |light| { *light = !*light; })
        }
    }

    fn apply_range<F>(&mut self, range: &Range, mut f: F) where F: FnMut(&mut bool) {
        for y in range.start.1..range.end.1 + 1 {
            for x in range.start.0..range.end.0 + 1 {
                f(&mut self.lights[(y * (LIGHTS_SIZE as i32) + x) as usize]);
            }
        }
    }

    fn count_lit(&self) -> usize {
        self.lights.iter().filter(|x| **x).count()
    }
}

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let instructions = s.lines().map(|x| Instruction::parse(x));

    let mut lights = Lights::new();

    for instruction in instructions {
        lights.apply(&instruction);
    }

    let lit_lights = lights.count_lit();

    println!("{} lights are lit :D", lit_lights);
}
