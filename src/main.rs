extern crate regex;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::iter::Iterator;
use std::collections::HashMap;
use regex::Regex;

enum Op {
    Plus,
    Minus,
    Times,
    Div,
}

enum Opr {
    LessThan,
    Eq,
}

enum I {
    Constant(String, i32),
    CopyVar(String, String),
    Oper(Op, String, String, String),
    Goto(i32),
    IfSt(String, Opr, i32),
}


fn main() {

    let mut state  = HashMap::new();

    let f = File::open("foo.txt").unwrap();
    let mut reader = BufReader::new(f);

    let lines: Vec<String> = reader.lines().filter_map(|result| result.ok()).collect();

    let mut line: usize = 0;
    let mut has_next: bool = line < lines.len();
    while has_next {
        let line_str = lines[line];

        println!("{}", line_str);

        let split: Vec<&str>  = line_str.split_whitespace().collect();

        let next: I;

        if split[0] == "goto" {
            let n = split[1].parse::<i32>().unwrap();
            next = I::Goto(n);
        } else if split[0] == "if" {

            let x = split[1].to_string();
            let n = split[3].parse::<i32>().unwrap();

            // need to parse the opr
            let opr: Opr = if split[2] == "<" {
                Opr::LessThan
            } else {
                Opr::Eq
            };

            next = I::IfSt(x, opr, n);
       
        } else if split[1] == "=" {
            // then this is either constant assigment, copy, or oper

            if split.len() > 3 {
                // then its a op
                // need to collect the +/-/*//
                let op: Op = match split[3] {
                    "+" => Op::Plus,
                    "-" => Op::Minus,
                    "*" => Op::Times,
                    "/" => Op::Div,
                    _   => Op::Plus,
                };
                let x = split[0].to_string();
                let y = split[2].to_string();
                let z = split[4].to_string();

                next = I::Oper(op, x, y, z);

            } else if split[2].parse::<i32>().is_ok() {
                // then it is a constant
                next = I::Constant(split[0].to_string(), split[2].parse::<i32>().unwrap())
            } else {
                // its a copy
                next = I::CopyVar(split[0].to_string(), split[2].to_string())
            }
        } else {
            println!("Not a valid instruction! Error!")
        }
        // Now, match on the type of oper.
        match next {
            I::Constant(x, val) => {
                state.insert(x, val);
            },
            I::CopyVar(x, y) => {
                let val = state.get(&y).unwrap();
                state.insert(x, *val);
            },
            I::Oper(op, x, y, z) => {
                let y_val = *state.get(&y).unwrap();
                let z_val = *state.get(&z).unwrap();

                let result = match op {
                    Op::Plus  => y_val + z_val,
                    Op::Minus => y_val - z_val,
                    Op::Times => y_val * z_val,
                    Op::Div   => y_val / z_val,
                };
                state.insert(x, result);
            },
            I::Goto(n) => {
                let goto_num = n as usize;
                line = goto_num - 1usize;
            },
            I::IfSt(x, opr, n) => {
                let x_val = *state.get(&x).unwrap();
                let result = match opr {
                    Opr::LessThan => x_val < 0,
                    Opr::Eq       => x_val == 0,
                };
                if result {
                    let goto_num = n as usize;
                    line = goto_num - 1usize;
                }
            },
        }
        line = line + 1usize;
        has_next = line < lines.len();
    }

    for (var, val) in &state {
        println!("{}:\t{}", var, val);
    }
}
    /*
    let x: Opr = Opr::LessThan;
    let y: Op  = Op::Plus;
    let z: I   = I::Oper(y, 5, 10, 15);
    
    match x {
        Opr::LessThan => println!("Hello Rust!"),
        Opr::Eq => println!("Rust Sucks!"),
    }

    
    match y {
        Op::Plus  => println!("Plus!"),
        Op::Minus => println!("Minus!"),
        Op::Times => println!("Times!"),
        Op::Div   => println!("Div!"),
    }

    match z {
        I::Constant(x, y)  => println!("Hello instruction!"),
        _ => println!("Bullshit")
    }

    // let x: I = I::Oper{ Plus, 5, 10, 9 };
    // let y: I = I::IfSt{ 10, Opr::LessThan, 5 };
    // println!("Hello, world!");
    // println!("{}", x);
    // println!("{}", y);

    */
