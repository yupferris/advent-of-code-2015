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

        let encoded_chars: i32 = x.chars().map(|x| match x {
            '"' | '\\' => 2,
            _ => 1
        }).sum();

        encoded_chars + 2 - char_num
    }).sum();

    println!("The answer is {} :D", answer);
}
