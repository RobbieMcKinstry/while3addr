extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::iter::Iterator;
use std::collections::HashMap;

// TODO turn this into a small step semantic model.
// Pass in the state, and return a configuration (state + line counter)
// Have this as a trait

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

    let mut state = HashMap::new();

    let f = File::open("foo.txt").unwrap();
    let reader = BufReader::new(f);

    let lines: Vec<String> = reader.lines().filter_map(|result| result.ok()).collect();

    let mut line: usize = 0;
    let mut has_next: bool = line < lines.len();
    while has_next {
        let line_str = lines[line].clone();

        println!("{}", line_str);

        let split: Vec<&str>  = line_str.split_whitespace().collect();

        let next: I = get_instruction(split);
        // Now, match on the type of oper.
        match next {
            I::Constant(x, val) => 
                exec_constant(&mut state, x, val),
            I::CopyVar(x, y) => {
                exec_copy(&mut state, x, y)
            },
            I::Oper(op, x, y, z) => {
                exec_oper(&mut state, op, x, y, z)
            },
            I::Goto(n) => {
                line = exec_goto(n); // gross
            },
            I::IfSt(x, opr, n) => {
                line = exec_if(&mut state, x, opr, n, line)
            },
        }

        line = line + 1usize;
        has_next = line < lines.len();
    }

    println!("\n");
    for (var, val) in &state {
        println!("{}:\t{}", var, val);
    }
}

// Determines which instruction there is to run
fn get_instruction(split: Vec<&str>) -> I {
    
    return if split[0] == "goto" {
        // then it must be a goto statement
        parse_goto(split)
    } else if split[0] == "if" {
        // then it must be an if statement
        parse_if(split)  
    } else if (split[1] == "=") && (split.len() > 3) { 
        // then its a op
        parse_op(split)
    } else if split[1] == "=" && split[2].parse::<i32>().is_ok() {
        // then it is a constant
        parse_constant(split)
    } else if split[1] == "=" {
        // its a copy
        parse_copy(split)
    } else {
        // then it is malformed syntax
        println!("Bad instruction!!!");
        I::Goto(0)
    };
}

fn parse_goto(split: Vec<&str>) -> I {
    let n = split[1].parse::<i32>().unwrap();
    I::Goto(n)
}

fn parse_if(split: Vec<&str>) -> I {
    let x = split[1].to_string();
    let n = split[5].parse::<i32>().unwrap();

    // need to parse the opr
    // since we're not sure what kind of relational operation it is yet
    let opr: Opr = if split[2] == "<" {
        Opr::LessThan
    } else {
        Opr::Eq
    };

    I::IfSt(x, opr, n)
}

fn parse_op(split: Vec<&str>) -> I {
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

    I::Oper(op, x, y, z)
}

fn parse_constant(split: Vec<&str>) -> I {
    I::Constant(split[0].to_string(), split[2].parse::<i32>().unwrap())
}

fn parse_copy(split: Vec<&str>) -> I {
    I::CopyVar(split[0].to_string(), split[2].to_string())
}

fn exec_constant(state: &mut HashMap<String, i32>, x: String, val: i32) {
    state.insert(x, val);
}

fn exec_copy(state: &mut HashMap<String, i32>, x: String, y: String) {
    let val = *state.get(&y).unwrap();
    state.insert(x, val);
}

fn exec_oper(state: &mut HashMap<String, i32>, op: Op,  x: String, y: String, z: String) {
    let y_val = *state.get(&y).unwrap();
    let z_val = *state.get(&z).unwrap();

    let result = match op {
        Op::Plus  => y_val + z_val,
        Op::Minus => y_val - z_val,
        Op::Times => y_val * z_val,
        Op::Div   => y_val / z_val,
    };
    state.insert(x, result);
}

fn exec_goto(n: i32) -> usize{
    let goto_num = n as usize;
    goto_num - 1usize
}

fn exec_if(state: &mut HashMap<String, i32>, x: String, opr: Opr, n: i32, line: usize) -> usize {
    let x_val = *state.get(&x).unwrap();
    let result = match opr {
        Opr::LessThan => x_val < 0,
        Opr::Eq       =>x_val == 0,
    };
    if result {
        let goto_num = n as usize;
        goto_num - 1usize
    } else {
        line
    }

}
