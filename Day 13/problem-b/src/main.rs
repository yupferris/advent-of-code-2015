use std::io::Read;
use std::fs::File;

use std::collections::{HashMap, HashSet};

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let mut relations = HashMap::new();
    for line in s.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let is_gain = parts[2] == "gain";
        let from = String::from(parts[0]);
        let to = String::from(parts[10].replace(".", ""));
        let mut amount = parts[3].parse::<i32>().unwrap();
        if !is_gain {
            amount = -amount;
        }

        if !relations.contains_key(&from) {
            relations.insert(from.clone(), HashMap::new());
        }
        let relation = relations.get_mut(&from).unwrap();
        relation.insert(to, amount);
    }

    let my_name = String::from("jake");
    let mut me = HashMap::new();
    for (name, relation) in relations.iter_mut() {
        me.insert(name.clone(), 0);
        relation.insert(my_name.clone(), 0);
    }
    relations.insert(my_name, me);

    for relation in relations.iter() {
        println!("{:?}", relation);
    }

    let mut best_happiness = None;

    for (name, _) in relations.iter() {
        println!("Starting with {}", name);

        visit_node(
            &relations,
            &mut best_happiness,
            name,
            Vec::new(),
            relations.keys().map(|x| x.clone()).collect::<HashSet<_>>());
    }

    println!("The best happiness is {:?} :D", best_happiness);
}

fn visit_node(
    nodes: &HashMap<String, HashMap<String, i32>>,
    best_happiness: &mut Option<i32>,
    node_name: &String,
    mut current_arrangement: Vec<String>,
    mut free_list: HashSet<String>) {

    if !free_list.contains(node_name) {
        return;
    }
    free_list.remove(node_name);

    current_arrangement.push(node_name.clone());

    if free_list.is_empty() {
        let mut current_happiness = 0;
        for i in 0..current_arrangement.len() {
            let j = if i == 0 { current_arrangement.len() - 1 } else { i - 1 };
            let k = if i == current_arrangement.len() - 1 { 0 } else { i + 1 };
            current_happiness += *nodes.get(&current_arrangement[i]).unwrap().get(&current_arrangement[j]).unwrap();
            current_happiness += *nodes.get(&current_arrangement[i]).unwrap().get(&current_arrangement[k]).unwrap();
        }

        *best_happiness = Some(match best_happiness {
            &mut Some(x) => {
                if current_happiness > x {
                    current_happiness
                } else {
                    x
                }
            },
            _ => current_happiness
        });

        return;
    }

    for (next_node_name, _) in nodes.get(node_name).unwrap() {
        visit_node(
            nodes,
            best_happiness,
            next_node_name,
            current_arrangement.clone(),
            free_list.clone());
    }
}
