use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum CommandError {
    EmptyInput,
    NotFound(String),
}

#[derive(Debug)]
pub enum CommandType {
    Exit,
    Echo,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::EmptyInput => write!(f, "The input was empty"),
            CommandError::NotFound(cmd) => write!(f, "{}: command not found", cmd),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub ty: CommandType,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(command: &str) -> Result<Self, CommandError> {
        let mut params = command.trim().split_whitespace();
        let name = params.next().ok_or(CommandError::EmptyInput)?;

        let ty = name.parse::<CommandType>()?;
        let args = params.map(|s| s.to_string()).collect();
        Ok(Command { ty, args })
    }
}

impl FromStr for CommandType {
    type Err = CommandError;

    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        match cmd {
            "exit" => Ok(CommandType::Exit),
            "echo" => Ok(CommandType::Echo),
            _ => Err(CommandError::NotFound(cmd.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_command() {
        let input = "banana banana";
        let result = Command::new(input);

        match result {
            Err(CommandError::NotFound(name)) => assert_eq!(name, "banana"),
            _ => panic!("Expected NotFound error, got {:?}", result),
        }
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = Command::new(input);

        assert!(matches!(result, Err(CommandError::EmptyInput)));
    }

    #[test]
    fn test_echo() {
        let input = "echo hello world";
        let cmd = Command::new(input).unwrap();

        assert!(matches!(cmd.ty, CommandType::Echo));
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }
}
