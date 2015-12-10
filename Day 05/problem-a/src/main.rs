use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let nice_count = s.lines().filter(|x| is_nice(&String::from(*x))).count();

    println!("{} strings are nice :D", nice_count);
}

fn is_nice(s: &String) -> bool {
    count_vowels(s) >= 3 &&
        has_repetition(s) &&
        !contains_invalid_combination(s)
}

fn count_vowels(s: &String) -> usize {
    s.chars().filter(|x| is_vowel(*x)).count()
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

fn has_repetition(s: &String) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    return false;
}

fn contains_invalid_combination(s: &String) -> bool {
    s.contains("ab") ||
        s.contains("cd") ||
        s.contains("pq") ||
        s.contains("xy")
}
