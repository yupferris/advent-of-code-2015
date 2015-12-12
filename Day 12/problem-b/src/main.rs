extern crate serde_json;

use std::io::Read;
use std::fs::File;

use serde_json::Value;

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let root: Value = serde_json::from_str(&s).unwrap();

    let sum = find_sum(&root);

    println!("The sum is {} :D", sum);
}

fn find_sum(value: &Value) -> i32 {
    match value {
        &Value::I64(x) => x as i32,
        &Value::U64(x) => x as i32,
        &Value::F64(x) => x as i32,
        &Value::Array(ref values) => values.iter().fold(0, |acc, x| acc + find_sum(x)),
        &Value::Object(ref map) => {
            let values = map.values().collect::<Vec<_>>();
            let contains_red = values.iter().any(|x| if x.is_string() {
                x.as_string().unwrap() == "red"
            } else {
                false
            });
            if contains_red {
                0
            } else {
                values.iter().fold(0, |acc, x| acc + find_sum(x))
            }
        },
        _ => 0
    }
}
