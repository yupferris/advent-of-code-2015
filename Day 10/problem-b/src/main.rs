fn main() {
    let mut sequence = String::from("1113122113");

    for _ in 0..50 {
        sequence = look_and_say(sequence);
    }

    println!("The length is {} :D", sequence.len());
}

fn look_and_say(s: String) -> String {
    let mut ret = String::new();

    let chars = s.chars().collect::<Vec<_>>();
    let mut index = 0;
    let mut current_run = None;
    while index < s.len() {
        let next_char = chars[index];
        current_run = match current_run {
            Some((c, length)) => {
                if next_char == c {
                    index += 1;
                    Some((next_char, length + 1))
                } else {
                    ret = ret + &format!("{}{}", length, c);
                    None
                }
            },
            None => {
                index += 1;
                Some((next_char, 1))
            }
        };
    }

    if let Some((c, length)) = current_run {
        ret = ret + &format!("{}{}", length, c);
    }

    ret
}

#[test]
fn a() {
    assert_eq!(look_and_say(String::from("1")), "11");
}

#[test]
fn b() {
    assert_eq!(look_and_say(String::from("11")), "21");
}

#[test]
fn c() {
    assert_eq!(look_and_say(String::from("21")), "1211");
}

#[test]
fn d() {
    assert_eq!(look_and_say(String::from("1211")), "111221");
}

#[test]
fn e() {
    assert_eq!(look_and_say(String::from("111221")), "312211");
}
