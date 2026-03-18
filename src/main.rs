mod command;
use std::io::{self, Write};

use crate::command::*;
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let cmd_result = Command::new(command.as_str());
        let cmd = match cmd_result {
            Ok(cmd) => cmd,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        match cmd.ty {
            CommandType::Exit => {
                break;
            }
            CommandType::Echo => {
                println!("{}", cmd.args.join(" "));
            }
        };
    }
}
