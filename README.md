# About

This is a tiny interpretter I wrote. It's not smart, it's not well coded, and that's okay with me.

Finals Week, Spring 2016: I really didn't feel like studying. So instead I spent 3 hours writing an interpreter in Rust, just to prove to myself I could. 

I hadn't used Rust in exactly a year. The last time I had used it, (which, mind you, was before 1.0 was released), I failed to complete the school project I was working I was using it for, because I couldn't figure out borrowing, mutability, and ownership. This time around, because I only had a few hours before I had to start studying for my impending final, I decided to **intentionally avoid all of the things that gave me trouble** the year prior. Thus, I intentionally wrote the entire interpreter in `main()` to avoid moving values and lifetime errors. Today, I finally broke my main method into smaller functions. This revealed a handful of weaknesses in my design. I don't know if it's worth it to fix them or to try to move onto a real compiler.

# Design

The design is tragic. I intentionally avoided build a proper lexer, or a proper parser. Instead, I manually pick out the tokens by examining the source lines directly. This is neither robust nor clever. For each line of source code, I re-parse the line every time I execute it. After parsing, I generate an AST node (which is an Instruction, of type I). Then, I push that AST node into a evaluator, which interprets the node and augments the global state. A smarter way to do this would be to take the array of strings (representing the source file) and convert them into an Vec<Instruction>. Then, evaluate each instruction directly, instead of reparsing each line every time.

I wonder if there's an EVEN SMARTER way of doing this with a REAL AST instead of my Vec<AST Nodes>. (I guess it's not really a **tree** if it's just a vector of instructions.) (PS Obviously there is a smarter way, I'm being intentionally dense.)

# Next Steps

I think I'm going to try to write a real compiler next. I'm going to try to build it correctly from the start, and I'm going to start with some up-front design work.

I'd like the compiler to support parallelism by component, have an AST represented entirely as an algebraic type, and be indentation sensitive, like Python. I'm only going to put in about 2 hours every week into this project, so don't expect much!
