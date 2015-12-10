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

        let mut chars_in_memory = 0;
        let mut index = 1;
        let chars = x.chars().collect::<Vec<_>>();
        while index < x.len() - 1 {
            match chars[index] {
                '\\' => {
                    index += 1;
                    match chars[index] {
                        'x' => {
                            index += 3;
                            chars_in_memory += 1;
                        },
                        _ => {
                            index += 1;
                            chars_in_memory += 1;
                        }
                    }
                },
                _ => {
                    chars_in_memory += 1;
                    index += 1;
                }
            }
        }

        char_num - chars_in_memory
    }).sum();

    println!("The answer is {} :D", answer);
}
