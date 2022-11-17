use ansi_term::{self, Colour};
use clap::Parser;
use cli_table::{format::Justify, Cell, Style, Table};
use inflector::Inflector;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::thread_rng;

#[derive(Parser)]
pub struct Cli {
    words: u8,
    #[clap(short, long)]
    numbers: bool,
    #[clap(short, long)]
    upper: bool,
}

fn generate(words: u8, numbers: bool, upper: bool) -> String {
    let str = include_str!("./words.txt").replace("-", "");
    let combined = str.split("\n").collect::<Vec<&str>>();
    let mut password: String = String::new();

    for _n in 0..words {
        let word = combined.choose(&mut rand::thread_rng()).expect("Failed to find a string");
        if upper {
            password.push_str(&word.to_title_case());
        } else {
            password.push_str(&word);

        }
    }

    if numbers {
        password.push_str(&thread_rng().gen_range(10000..9999999).to_string())
    }

    return password;
}

fn main() {
    let args = Cli::parse();
    let password = generate(args.words, args.numbers, args.upper);
    let table = vec![
        vec!["Password: ".cell(), Colour::Green.paint(password).cell().justify(Justify::Right)]
    ].table()
    .title(vec![
        "PassGen".cell().bold(true),
        "".cell()
    ]);
    println!("{}", table.display().unwrap());
}
