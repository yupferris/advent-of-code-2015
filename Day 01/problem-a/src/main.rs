use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let floor =
        s.chars()
        .fold(0, |acc, x| match x {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => unreachable!()
        });

    println!("This input will take santa to floor {}! :D", floor);
}
