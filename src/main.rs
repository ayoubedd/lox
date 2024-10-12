use std::{env, process::exit};

use tokenizer::tokenize;
mod tokenizer;

fn repl() {
    let stdin = std::io::stdin();
    let mut line = String::new();

    loop {
        let wow = stdin.read_line(&mut line);
        match wow {
            Ok(line_size) => {
                if line_size == 0 {
                    break;
                }
                let tokens = tokenize(&line[..], "repl");
                dbg!(&tokens);
            }
            Err(_err) => {
                exit(0);
            }
        }
        line.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // REPL mode
        1 => repl(),

        // Input file provided
        2 => {
            let input = std::fs::read_to_string(&args[1]).expect("faillure reading input file");
            let tokens = tokenizer::tokenize(&input, &args[1]);
            dbg!(&tokens);
        }

        _ => {
            println!("usage: {} main.lox", args[0]);
            exit(1);
        }
    }
}
