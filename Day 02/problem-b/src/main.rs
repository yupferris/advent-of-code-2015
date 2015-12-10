use std::io::Read;
use std::fs::File;

use std::cmp::max;

struct Present {
    l: i32,
    w: i32,
    h: i32
}

impl Present {
    fn required_ribbon_amount(&self) -> i32 {
        let largest_side = max(max(self.l, self.w), self.h);

        let wrap_amount = (self.l + self.w + self.h - largest_side) * 2;

        let bow_amount = self.l * self.w * self.h;

        wrap_amount + bow_amount
    }
}

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let lines = s.lines();
    let presents = lines.map(|line| {
        let string_components = line.split('x');
        let int_components = string_components.map(|comp| comp.parse::<i32>().unwrap()).collect::<Vec<_>>();
        Present {
            l: int_components[0],
            w: int_components[1],
            h: int_components[2]
        }
    });
    let required_ribbon_amount = presents.fold(0, |acc, present| {
        acc + present.required_ribbon_amount()
    });

    println!("The elves should order {} feet of ribbon :D", required_ribbon_amount);
}
