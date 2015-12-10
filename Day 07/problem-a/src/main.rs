#[macro_use]
extern crate nom;

use std::io::Read;
use std::fs::File;
use std::str;
use std::str::FromStr;

use nom::{digit, multispace, alpha};

#[derive(Debug)]
enum Op {
    And,
    Or,
    LShift,
    RShift
}

#[derive(Debug)]
enum Expr {
    Const(u16),
    Ref(String),
    NotOp(Box<Expr>),
    BinOp(Op, Box<Expr>, Box<Expr>)
}

#[derive(Debug)]
struct Connection {
    input: Box<Expr>,
    output: String
}

named!(connection_parser<Connection>,
       chain!(
           input: expr_parser ~
               delimited!(opt!(multispace), tag!("->"), opt!(multispace)) ~
               output: string_parser,
           || { Connection {
               input: Box::new(input),
               output: output
           } }));

named!(string_parser<String>,
       map_res!(
           map_res!(alpha, str::from_utf8),
           FromStr::from_str));

named!(expr_parser<Expr>,
       alt!(
           expr_not_op_parser |
           expr_bin_op_parser |
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

    let connections = s.lines().map(|x| connection_parser(x.as_bytes())).collect::<Vec<_>>();
    for c in connections {
        println!("{:?}", c);
    }
}
