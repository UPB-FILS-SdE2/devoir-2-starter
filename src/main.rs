use std::io;
use std::io::Write;

mod parser;

use parser::CommandParser;

fn main() {
    loop {
        print!("$ ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let cmd = line.trim_end();
        println!("{:?}", cmd);
        let command = CommandParser::new().parse(cmd);
        match command {
            Ok(parse_command) => println!("{:#?}", parse_command),
            Err(error) => eprintln!("Parse error: {}", error),
        }
    }
}
