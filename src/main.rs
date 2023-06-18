mod tokenizer;
use anyhow::Result;
use clap::Parser;
use std::{fs, path::PathBuf};
use zinc::{
    tokenizer::Tokenizer,
    wayfarer::{self, Wayfarer},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    pub filename: PathBuf,

    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<()> {
    let command = Command::parse();

    let filename = command.filename;

    let file = fs::read_to_string(filename)?;

    let mut tokenizer = Tokenizer::default();

    tokenizer.tokenize(file)?;

    Wayfarer::new(tokenizer).generate()?;

    Ok(())
}
