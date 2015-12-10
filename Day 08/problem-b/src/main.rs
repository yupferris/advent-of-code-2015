#![feature(iter_arith)]

use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let answer: i32 = s.lines().map(|x| {
        let char_num = x.len() as i32;

        let mut encoded_chars = 2;
        let mut index = 0;
        let chars = x.chars().collect::<Vec<_>>();
        while index < x.len() {
            match chars[index] {
                '"' | '\\' => {
                    encoded_chars += 2;
                    index += 1;
                },
                _ => {
                    encoded_chars += 1;
                    index += 1;
                }
            }
        }

        encoded_chars - char_num
    }).sum();

    println!("The answer is {} :D", answer);
}
