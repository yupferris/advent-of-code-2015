use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let nice_count = s.lines().filter(|x| is_nice(&String::from(*x))).inspect(|x| println!("{} is nice", x)).count();

    println!("{} strings are nice :D", nice_count);
}

fn is_nice(s: &String) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    has_repeated_non_overlapping_pair(&chars) &&
        has_repetition_with_letter_between(&chars)
}

fn has_repeated_non_overlapping_pair(chars: &Vec<char>) -> bool {
    for i in 0..chars.len() - 1 {
        let pair_start = i;
        let pair_end = i + 1;
        let pair = (chars[i], chars[i + 1]);
        if range_contains_pair(chars, 0, pair_start, &pair) || range_contains_pair(chars, pair_end + 1, chars.len(), &pair) {
            return true;
        }
    }
    return false;
}

fn range_contains_pair(chars: &Vec<char>, range_start: usize, range_end: usize, pair: &(char, char)) -> bool {
    if range_start >= range_end {
        return false;
    }
    for i in range_start..range_end - 1 {
        if chars[i] == pair.0 && chars[i + 1] == pair.1 {
            return true;
        }
    }
    return false;
}

fn has_repetition_with_letter_between(chars: &Vec<char>) -> bool {
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    return false;
}
