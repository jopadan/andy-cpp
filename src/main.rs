mod lexer;

use crate::lexer::{Lexer, LexerError};
use clap::Parser;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Andy C++")]
#[command(author = "Tim Fennis <fennis.tim@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "An interpreter for the Andy C++ language")]
struct Cli {
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if let Some(path) = cli.file {
        let mut file = File::open(path)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        println!("{}", run(&string)?);
    } else {
        let mut line = String::new();
        loop {
            // print prompt
            print!("λ ");
            std::io::stdout().flush()?;

            // Read a line
            let len = std::io::stdin().read_line(&mut line)?;

            // If the line was empty (from pressing CTRL+D) we quit
            if len == 0 {
                break;
            }

            // Print the response from the interpreter
            match run(&line) {
                Ok(output) => println!("{}", output),
                Err(err) => eprintln!("Error: {}", err),
            }

            line.clear();
        }
        println!("Bye!");
    }

    Ok(())
}

#[derive(Debug)]
enum InterpreterError {
    LexerError { cause: LexerError },
}

fn run(input: &str) -> Result<String, InterpreterError> {
    let scanner = Lexer::from_str(input);
    for token in scanner {
        println!("{:?}", token?);
    }

    Ok(String::from(""))
}

impl From<LexerError> for InterpreterError {
    fn from(value: LexerError) -> Self {
        InterpreterError::LexerError { cause: value }
    }
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::LexerError { cause } => write!(f, "Scanner error: {cause}"),
        }
    }
}

impl Error for InterpreterError {}

#[cfg(test)]
mod test {
    use crate::Cli;
    use clap::CommandFactory;

    #[test]
    fn test_clap() {
        Cli::command().debug_assert();
    }
}
