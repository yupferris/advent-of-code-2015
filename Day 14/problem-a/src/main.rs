use std::io::Read;
use std::fs::File;

use std::cmp::max;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32
}

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let reindeer = s.lines().map(|x| {
        let parts = x.split(' ').collect::<Vec<_>>();
        Reindeer {
            name: String::from(parts[0]),
            speed: parts[3].parse::<i32>().unwrap(),
            fly_time: parts[6].parse::<i32>().unwrap(),
            rest_time: parts[13].parse::<i32>().unwrap(),
        }
    }).collect::<Vec<_>>();

    /*for d in reindeer {
        println!("{:?}", d);
}*/

    let time = 2503;
    let farthest = reindeer.iter().fold(0, |acc, x| {
        let mut distance = 0;
        let mut is_flying = true;
        let mut state_time = 0;
        for i in 0..time {
            state_time += 1;

            if is_flying {
                distance += x.speed;

                if state_time >= x.fly_time {
                    state_time = 0;
                    is_flying = false;
                }
            } else {
                if state_time >= x.rest_time {
                    state_time = 0;
                    is_flying = true;
                }
            }
        }

        max(acc, distance)
    });

    println!("The farthest was {} :D", farthest);
}
