use std::io::Read;
use std::fs::File;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Edge {
    to: String,
    length: i32
}

type Node = HashSet<Edge>;
type Nodes = HashMap<String, Node>;

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let mut nodes = HashMap::new();
    for line in s.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let node_name = String::from(parts[0]).clone();
        let edge = Edge {
            to: String::from(parts[2]),
            length: parts[4].parse::<i32>().unwrap()
        };
        ensure_node(&mut nodes, &node_name);
        let node = nodes.get_mut(&node_name).unwrap();
        node.insert(edge);
    }

    for (node_name, node) in nodes.clone().iter() {
        for edge in node.iter() {
            ensure_node(&mut nodes, &edge.to);
            let node = nodes.get_mut(&edge.to).unwrap();
            node.insert(Edge {
                to: node_name.clone(),
                length: edge.length
            });
        }
    }

    let mut best_distance = None;
    for (name, _) in nodes.iter() {
        visit_node(
            &nodes,
            &mut best_distance,
            0,
            &name,
            nodes.keys().map(|x| x.clone()).collect::<HashSet<_>>());
    }

    println!("The best distance is {:?} :D", best_distance);
}

fn ensure_node(nodes: &mut Nodes, name: &String) {
    if !nodes.contains_key(name) {
        nodes.insert(name.clone(), HashSet::new());
    }
}

fn visit_node(
    nodes: &Nodes,
    best_distance:
    &mut Option<i32>,
    current_distance: i32,
    node_name: &String,
    mut free_list: HashSet<String>) {

    if !free_list.contains(node_name) {
        return;
    }
    free_list.remove(node_name);

    if free_list.is_empty() {
        *best_distance = Some(match best_distance {
            &mut Some(x) => {
                if current_distance > x {
                    current_distance
                } else {
                    x
                }
            },
            _ => current_distance
        });

        return;
    }

    for edge in nodes.get(node_name).unwrap() {
        visit_node(
            nodes,
            best_distance,
            current_distance + edge.length,
            &edge.to,
            free_list.clone());
    }
}
