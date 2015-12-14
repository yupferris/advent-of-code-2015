use std::io::Read;
use std::fs::File;

use std::cmp::max;

#[derive(Debug)]
struct Reindeer {
    name: String,
    is_flying: bool,
    state_time: i32,
    distance: i32,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
    points: i32
}

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let mut reindeer = s.lines().map(|x| {
        let parts = x.split(' ').collect::<Vec<_>>();
        Reindeer {
            name: String::from(parts[0]),
            is_flying: true,
            state_time: 0,
            distance: 0,
            speed: parts[3].parse::<i32>().unwrap(),
            fly_time: parts[6].parse::<i32>().unwrap(),
            rest_time: parts[13].parse::<i32>().unwrap(),
            points: 0
        }
    }).collect::<Vec<_>>();

    let time = 2503;
    for _ in 0..time {
        let mut best_distance = 0;

        for d in reindeer.iter_mut() {
            d.state_time += 1;

            if d.is_flying {
                d.distance += d.speed;

                if d.state_time >= d.fly_time {
                    d.state_time = 0;
                    d.is_flying = false;
                }
            } else {
                if d.state_time >= d.rest_time {
                    d.state_time = 0;
                    d.is_flying = true;
                }
            }

            best_distance = max(best_distance, d.distance);
        }

        for d in reindeer.iter_mut() {
            if d.distance == best_distance {
                d.points += 1;
            }
        }
    }

    let most_points = reindeer.iter().fold(0, |acc, x| {
        max(acc, x.points)
    });

    println!("The most points was {} :D", most_points);
}
