use std::{process::Command, fmt::Display};


#[derive(Clone, Copy)]
pub enum ArgType {
    ArgType_String,
    ArgType_u32,
}

#[derive(Clone)]
pub struct ArgContainer {
    pub args_string: Vec<String>,
    pub args_u32: Vec<u32>,
}

impl ArgContainer {
    pub fn new() -> ArgContainer {
        ArgContainer {
            args_string: Vec::new(),
            args_u32: Vec::new(),
        }
    }
}

pub enum CommandError {
    NoCommandRecieved,
    CommandNotFound,
    IncorrectNumberOfArguments,
    InvalidArgumentType,
}

pub struct CommandParser<T: Copy> {
    registered_cmds: Vec<RegisteredCommand<T>>,
    description: String,
}

impl<T: Copy> CommandParser<T> {
    pub fn new() -> CommandParser<T> {
        CommandParser { registered_cmds: Vec::new(), description: String::new() }
    }

    pub fn from(cmds: Vec<RegisteredCommand<T>>) -> CommandParser<T> {
        CommandParser { registered_cmds: cmds, description: String::new() }
    }

    pub fn register_cmd(&mut self, cmd: RegisteredCommand<T>) {
        self.registered_cmds.push(cmd)
    }

    pub fn register_cmds(&mut self, cmds: Vec<RegisteredCommand<T>>) {
        self.registered_cmds = cmds
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_help_text(&self) -> String {
        let mut output = String::new();

        output += self.description.as_str();
        output += "\n\n";

        for cmd in &self.registered_cmds {
            output += format!("{}\n\n", cmd).as_str();
        }

        output
    }

    pub fn parse_string(&self, string_input: String) -> Result<ParsedCommand<T>, CommandError> {
        self.parse_vec(
            string_input
                .trim()
                .split_whitespace()
                .map(|s| String::from(s)).collect()
        )
    }

    pub fn parse_vec(&self, vec_input: Vec<String>) -> Result<ParsedCommand<T>, CommandError> {
        if vec_input.len() > 0 {
            for rcmd in &self.registered_cmds {
                if rcmd.is_cmd(&vec_input[0]) {
                    return self.parse_cmd(rcmd, &vec_input[1..]);
                }
            }
            return Err(CommandError::CommandNotFound);
        }
        else {
            return Err(CommandError::NoCommandRecieved);
        }
    }

    fn parse_cmd(&self, rcmd: &RegisteredCommand<T>, input_args: &[String]) -> Result<ParsedCommand<T>, CommandError> {
        if input_args.len() >= rcmd.num_args as usize {
            if let Some(argt) = &rcmd.arg_type {
                let mut arg_container = ArgContainer::new();
                match argt {
                    ArgType::ArgType_String => {
                        for i in 0..rcmd.num_args {
                            arg_container.args_string.push(String::from(input_args[i as usize].as_str()));
                        }
                    },
                    ArgType::ArgType_u32 => {
                        for i in 0..rcmd.num_args {
                            match input_args[i as usize].parse::<u32>() {
                                Ok(v) => arg_container.args_u32.push(v),
                                Err(e) => {
                                    return Err(CommandError::InvalidArgumentType);
                                }
                            }
                        }
                    }
                }
                return Ok(ParsedCommand::new(rcmd.cmd_id, rcmd.arg_type, Some(arg_container)));
            }
            else {
                return Ok(ParsedCommand::new(rcmd.cmd_id, None, None));
            }
        }
        else {
            // Partial arguments
            if input_args.len() != 0 {
                return Err(CommandError::IncorrectNumberOfArguments);
            }

            // Use default arguments
            if let Some(def_args) = rcmd.get_default_arguments() {
                return Ok(ParsedCommand::new(rcmd.cmd_id, rcmd.arg_type, Some(def_args.clone())));
            }
            else {
                // No default arguments available. User must supply all args.
                return Err(CommandError::IncorrectNumberOfArguments);
            }
        }
    }
}

pub struct ParsedCommand<T: Copy> {
    cmd_id: T,
    arg_type: Option<ArgType>,
    args: Option<ArgContainer>,
}

impl<T: Copy> ParsedCommand<T> {
    pub fn new(cmd_id: T, arg_type: Option<ArgType>, args: Option<ArgContainer>) -> ParsedCommand<T> {
        ParsedCommand {
            cmd_id,
            arg_type,
            args,
        }
    }

    pub fn get_id(&self) -> &T {
        &self.cmd_id
    }

    pub fn get_arg_type(&self) -> &Option<ArgType> {
        &self.arg_type
    }

    pub fn get_args_string(&self) -> Option<&Vec<String>> {
        if let Some(arg_container) = &self.args {
            return Some(&arg_container.args_string)
        }
        None
    }

    pub fn get_args_u32(&self) -> Option<&Vec<u32>> {
        if let Some(arg_container) = &self.args {
            return Some(&arg_container.args_u32)
        }
        None
    }
}

pub struct RegisteredCommand<T: Copy> {
    cmd_aliases: Vec<String>,
    cmd_id: T,
    num_args: u32,
    arg_type: Option<ArgType>,
    help_str: String,
    default_args: Option<ArgContainer>,
}

impl<T: Copy> Display for RegisteredCommand<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        // Show aliases
        output += "Command aliases: [";
        for i in 0..self.cmd_aliases.len() {
            output.push_str(self.cmd_aliases[i].as_str());
            if i != self.cmd_aliases.len() - 1 {
                output += ",";
            }
        }
        output += "]\n";

        // Show number of args
        if self.num_args > 0 {
            output += format!("Number of arguments: {}\n", self.num_args).as_str();
        }

        // Show argument type
        if let Some(arg_type) = self.arg_type {
            let at = match arg_type {
                ArgType::ArgType_String => "String",
                ArgType::ArgType_u32 => "u32",
            };

            output += format!("Argument Type: {}\n", at).as_str();
        }

        // Show help
        output += self.help_str.as_str();
        output += "\n";


        write!(f, "{}", output)
    }
}

impl<T: Copy> RegisteredCommand<T> {
    pub fn new(id: T) -> RegisteredCommandBuilder<T> {
        RegisteredCommandBuilder::new(id)
    }

    pub fn is_cmd(&self, other_str: &String) -> bool {
        for alias in &self.cmd_aliases {
            if alias == other_str.as_str() {
                return true;
            }
        }
        false
    }

    pub fn get_default_arguments(&self) -> &Option<ArgContainer> {
        &self.default_args
    }
}

pub struct RegisteredCommandBuilder<T: Copy> {
    cmd_aliases: Vec<String>,
    cmd_id: T,
    num_args: u32,
    arg_type: Option<ArgType>,
    help_str: String,
    default_args: Option<ArgContainer>,
}

impl<T: Copy> RegisteredCommandBuilder<T> {
    pub fn new(id: T) -> RegisteredCommandBuilder<T> {
        RegisteredCommandBuilder {
            cmd_aliases: Vec::new(),
            cmd_id: id,
            num_args: 0,
            arg_type: None,
            help_str: String::new(),
            default_args: None,
        }
    }

    pub fn add_aliases(mut self, cmd_aliases: &[&str]) -> RegisteredCommandBuilder<T> {
        self.cmd_aliases.append(&mut cmd_aliases.iter().map(|c| String::from(*c)).collect());
        self
    }

    pub fn add_num_args(mut self, n: u32) -> RegisteredCommandBuilder<T> {
        self.num_args = n;
        self
    }

    pub fn add_arg_type(mut self, arg_type: ArgType) -> RegisteredCommandBuilder<T> {
        self.arg_type = Some(arg_type);
        self
    }

    pub fn add_help_str(mut self, help: &str) -> RegisteredCommandBuilder<T> {
        self.help_str = String::from(help);
        self
    }

    pub fn add_help_string(mut self, help: String) -> RegisteredCommandBuilder<T> {
        self.help_str = help;
        self
    }

    pub fn add_default_args_string(mut self, def: Vec<String>) -> RegisteredCommandBuilder<T> {
        let mut arg_container = ArgContainer::new();
        arg_container.args_string = def;
        self.default_args = Some(arg_container);
        self
    }

    pub fn add_default_args_u32(mut self, def: Vec<u32>) -> RegisteredCommandBuilder<T> {
        let mut arg_container = ArgContainer::new();
        arg_container.args_u32 = def;
        self.default_args = Some(arg_container);
        self
    }

    pub fn build(self) -> Option<RegisteredCommand<T>> {
        Some(RegisteredCommand {
            cmd_aliases: self.cmd_aliases,
            cmd_id: self.cmd_id,
            num_args: self.num_args,
            arg_type: self.arg_type,
            help_str: self.help_str,
            default_args: self.default_args,
        })
    }
}


// === UNIT TESTS ===

#[cfg(test)]
mod tests {
    use super::{RegisteredCommand, CommandParser};

    #[derive(Clone, Copy)]
    enum TestCommandEnum {
        TestCommandOne,
        TestCommandTwo,
        TestCommandThree,
    }
}