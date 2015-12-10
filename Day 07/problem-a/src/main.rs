#![feature(slice_patterns)]

#[macro_use]
extern crate nom;

use std::io::Read;
use std::fs::File;
use std::str;
use std::str::FromStr;

use std::collections::HashMap;

use nom::{IResult, digit, multispace, alpha};

enum Op {
    And,
    Or,
    LShift,
    RShift
}

enum Expr {
    Const(u16),
    Ref(String),
    NotOp(Box<Expr>),
    BinOp(Op, Box<Expr>, Box<Expr>)
}

type Connection = (String, Box<Expr>);

named!(connection_parser<Connection>,
       chain!(
           input: expr_parser ~
               delimited!(opt!(multispace), tag!("->"), opt!(multispace)) ~
               name: string_parser,
           || { (name, Box::new(input)) }));

named!(string_parser<String>,
       map_res!(
           map_res!(alpha, str::from_utf8),
           FromStr::from_str));

named!(expr_parser<Expr>,
       alt!(
           expr_not_op_parser |
           complete!(expr_bin_op_parser) |
           expr_term_parser));

named!(expr_not_op_parser<Expr>,
       chain!(
           tag!("NOT") ~
               value_expr: expr_term_parser,
           || { Expr::NotOp(Box::new(value_expr)) }));

named!(expr_bin_op_parser<Expr>,
       chain!(
           lhs: expr_term_parser ~
               op: expr_op_parser ~
               rhs: expr_term_parser,
           || { Expr::BinOp(op, Box::new(lhs), Box::new(rhs)) }));

named!(expr_op_parser<Op>,
       delimited!(
           opt!(multispace),
           alt!(
               chain!(tag!("AND"), || { Op::And }) |
               chain!(tag!("OR"), || { Op::Or }) |
               chain!(tag!("LSHIFT"), || { Op::LShift }) |
               chain!(tag!("RSHIFT"), || { Op::RShift })),
           opt!(multispace)));

named!(expr_term_parser<Expr>,
       alt!(
           expr_const_parser |
           expr_ref_parser));

named!(expr_const_parser<Expr>,
       delimited!(
           opt!(multispace),
           chain!(value: u16_parser, || { Expr::Const(value) }),
           opt!(multispace)));

named!(
    u16_parser<u16>,
    map_res!(
        map_res!(digit, str::from_utf8),
        FromStr::from_str));

named!(expr_ref_parser<Expr>,
       delimited!(
           opt!(multispace),
           chain!(name: string_parser, || { Expr::Ref(name) }),
           opt!(multispace)));

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let s = s;

    let connections = s.lines().map(|x| {
        match connection_parser(x.as_bytes()) {
            IResult::Done([], c) => c,
            _ => unreachable!()
        }
    }).collect::<HashMap<_, _>>();

    let mut wires = HashMap::new();
    for (name, _) in connections.iter() {
        wires.insert(name.clone(), None);
    }

    loop {
        let mut finished = true;

        for (name, expr) in connections.iter() {
            if let &None = wires.get(name).unwrap() {
                match evaluate_expr(&wires, &connections, expr) {
                    None => { finished = false; },
                    x => { *wires.get_mut(name).unwrap() = x; }
                }
            }
        }

        if finished {
            break;
        }
    }

    println!("The value on wire a is {} :D", wires.get("a").unwrap().unwrap());
}

fn evaluate_expr(wires: &HashMap<String, Option<u16>>, connections: &HashMap<String, Box<Expr>>, expr: &Expr) -> Option<u16> {
    match expr {
        &Expr::Const(x) => Some(x),
        &Expr::Ref(ref name) => wires.get(name).unwrap().clone(),
        &Expr::NotOp(ref sub_expr) => evaluate_expr(wires, connections, sub_expr).map(|x| !x),
        &Expr::BinOp(ref op, ref lhs, ref rhs) => {
            match (evaluate_expr(wires, connections, lhs), evaluate_expr(wires, connections, rhs)) {
                (Some(lhs_value), Some(rhs_value)) => Some(match op {
                    &Op::And => lhs_value & rhs_value,
                    &Op::Or => lhs_value | rhs_value,
                    &Op::LShift => lhs_value << rhs_value,
                    &Op::RShift => lhs_value >> rhs_value
                }),
                _ => None
            }
        }
    }
}
