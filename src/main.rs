use std::io;
use std::io::Write;

mod parser;

use parser::ast::{Command, Word, Words};

use parser::CommandParser;

/// Returns one single String by combining all
/// the words of a parameter
fn read_word(words: &Box<Words>) -> String {
    let mut parameter = String::new();
    for word in words.iter() {
        // word is &&Box<...>, ** is Box<...>
        match **word {
            // Word::Word(s) moves the String s out of the Word(s)
            // ref s means s is a refernece to the String s from Word(s)
            // s is a reference to String(&String)
            Word::Word(ref s) => parameter.push_str(s),
            Word::Expand(ref v) => parameter.push_str(""/*replace this with the value of environment variable (std::env::var) who's name is stored in v or "" if the variable is not defined*/),
            // add here the other Word posibilities
            _ => {}
        }
    }
    parameter
}

fn run(command: &Box<Command>) {
    // *command is Box<Command>
    // **command is Command
    match **command {
        // we use ref here has we cannot move parameters out of
        // the command, so we take a reference to it
        Command::SimpleCommand { ref parameters, .. } => {
            let command = read_word(&parameters[0]);
            match command.as_str() {
                "exit" => std::process::exit(0),
                command => println!("run {}", command),
            }
        }
        _ => unimplemented!(),
    }
}

fn main() {
    loop {
        print!("$ ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let cmd = line.trim_end();
        // println!("{:?}", cmd);
        let command = CommandParser::new().parse(cmd);
        match command {
            Ok(parse_command) => {
                println!("{:#?}", parse_command);
                run(&parse_command);
            }
            Err(error) => eprintln!("Parse error: {}", error),
        }
    }
}
