use std::{env, io};

fn process_input<R>(input: R) where R: io::Read {
    let mut input = input;
    let mut filter_str = String::new();

    if let Err(x) = input.read_to_string(&mut filter_str) {
        panic!("Can't read: {}", x);
    }
    let mut parser = rql_parser::parser::Parser::new_from_string(filter_str);
    let parsed = match parser.parse_query() {
        Ok(query) => query,
        Err(e) => panic!("parse filter: {:?}", e),
    };
    println!("out: {}", parsed)
}

fn main() {
    let mut args: Vec<_> = env::args().collect();
    let str = args.remove(1);
    if str == "" || str == "-" {
        process_input(io::stdin());
    } else {
        process_input(str.as_bytes());
    }
}