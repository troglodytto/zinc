#![deny(clippy::pedantic)]
use std::{fmt::Display, io::Read, path::Path, process::exit};
use zinc::{ast::AbstractSyntaxTree, tokenizer::Tokenizer, wayfarer::Wayfarer};

fn read_source_file<P>(path: P) -> String
where
    P: AsRef<Path> + Display + Copy,
{
    let mut contents = String::new();

    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => {
            eprint!("Failed to open file {path}: {error}");
            exit(-1);
        }
    };

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(error) => {
            eprint!("Failed to open file {path}: {error}");
            exit(-1);
        }
    };

    contents
}

fn main() {
    let source = read_source_file("ast-test.zn");
    let tokens = Tokenizer::tokenize(&source);

    let mut wayfarer = Wayfarer::new(tokens);

    let abstract_syntax_tree = AbstractSyntaxTree::parse(&mut wayfarer);

    // println!("{:?}", wayfarer.next());

    // wayfarer.next();

    // println!("{:?}", wayfarer.peek_prev());
    // println!("{:?}", wayfarer.prev());

    // for token in wayfarer {
    //     print!("{token}");
    // }

    // let parser = Parser;

    // parser.parse(tokens);
}
