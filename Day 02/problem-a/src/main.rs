use std::io::Read;
use std::fs::File;

use std::cmp::min;

struct Present {
    l: i32,
    w: i32,
    h: i32
}

impl Present {
    fn required_paper_amount(&self) -> i32 {
        let side1 = self.l * self.w;
        let side2 = self.w * self.h;
        let side3 = self.h * self.l;

        let smallest_side = min(min(side1, side2), side3);

        let surface_area = 2 * side1 + 2 * side2 + 2 * side3;

        surface_area + smallest_side
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
    let required_paper_amount = presents.fold(0, |acc, present| {
        acc + present.required_paper_amount()
    });

    println!("The elves should order {} square feet of wrapping paper :D", required_paper_amount);
}
