use std::{fmt::Display, io::Read, path::Path, process::exit};
use zinc::tokenizer::Tokenizer;

fn read_source_file<P>(path: P) -> String
where
    P: AsRef<Path> + Display + Copy,
{
    let mut contents = String::new();

    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => {
            eprint!("Failed to open file {path}: {}", error);
            exit(-1);
        }
    };

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(error) => {
            eprint!("Failed to open file {path}: {}", error);
            exit(-1);
        }
    };

    return contents;
}

fn main() {
    let source = read_source_file("source.zn");
    let tokens = Tokenizer::tokenize(source);

    for token in &tokens {
        print!("{token}");
        // dbg!(token);
    }

    // let parser = Parser;

    // parser.parse(tokens);
}
