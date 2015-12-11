fn main() {
    let input = String::from("hepxcrrq");

    let password = next_password(&next_password(&input));

    println!("Santa's next next password is {} :D", password);
}

fn next_password(s: &String) -> String {
    let mut ret = s.clone();
    loop {
        ret = increment_password(&ret);
        if is_valid(&ret) {
            break;
        }
    }
    ret
}

fn increment_password(s: &String) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    let mut carry = true;
    for i in 0..chars.len() {
        let index = chars.len() - 1 - i;
        let mut c = chars[index] as u8;
        if carry {
            c += 1;
        }
        carry = match c {
            0x7b => { // ascii 'z' + 1
                c = 0x61; // ascii 'a'
                true
            },
            _ => false
        };
        chars[index] = c as char;
    }
    String::from_utf8(chars.iter().map(|x| *x as u8).collect::<Vec<_>>()).unwrap()
}

fn is_valid(s: &String) -> bool {
    has_straight(s) &&
        !contains_invalid_chars(s) &&
        contains_two_pairs(s)
}

fn has_straight(s: &String) -> bool {
    let chars = s.chars().map(|x| x as i32).collect::<Vec<_>>();
    for i in 0..s.len() - 2 {
        let c = chars[i];
        if chars[i + 1] == c + 1 && chars[i + 2] == c + 2 {
            return true;
        }
    }
    return false;
}

fn contains_invalid_chars(s: &String) -> bool {
    s.contains('i') || s.contains('o') || s.contains('l')
}

fn contains_two_pairs(s: &String) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    for i in 0..s.len() - 1 {
        if chars[i] == chars[i + 1] {
            for j in i + 2..s.len() - 1 {
                if chars[j] == chars[j + 1] {
                    return true;
                }
            }
        }
    }
    return false;
}
