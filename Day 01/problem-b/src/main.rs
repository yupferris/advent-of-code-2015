use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let mut current_floor = 0;
    let mut position = 1;
    for c in s.chars() {
        match c {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => unreachable!()
        };

        if current_floor < 0 {
            break;
        }

        position += 1;
    }

    println!("Santa will enter the basement at position {}! :D", position);
}
