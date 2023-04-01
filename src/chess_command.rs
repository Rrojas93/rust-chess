use std::process::Command;


pub enum ArgType {
    ArgType_String,
    ArgType_u32,
}

struct ArgContainer {
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

pub struct CommandParser<T> {
    registered_cmds: Vec<RegisteredCommand<T>>,
}

impl<T> CommandParser<T> {
    pub fn new() -> CommandParser<T> {
        CommandParser { registered_cmds: Vec::new() }
    }

    pub fn from(cmds: Vec<RegisteredCommand<T>>) -> CommandParser<T> {
        CommandParser { registered_cmds: cmds }
    }

    pub fn register_cmd(&mut self, cmd: RegisteredCommand<T>) {
        self.registered_cmds.push(cmd)
    }

    pub fn register_cmds(&mut self, cmds: Vec<RegisteredCommand<T>>) {
        self.registered_cmds = cmds
    }

    pub fn parse_string(&self, string_input: String) -> Option<ParsedCommand<T>> {
        self.parse_vec(
            string_input
                .trim()
                .split_whitespace()
                .map(|s| String::from(s)).collect()
        )
    }

    pub fn parse_vec(&self, vec_input: Vec<String>) -> Option<ParsedCommand<T>> {
        if vec_input.len() > 0 {
            for rcmd in self.registered_cmds {
                if rcmd.is_cmd(vec_input[0]) {
                    return self.parse_cmd(rcmd, &vec_input[1..]);
                }
            }
        }
        else {
            // TODO: No input. Print/return help.
        }
        None
    }

    pub fn parse_cmd(&self, rcmd: RegisteredCommand<T>, input_args: &[String]) -> Option<ParsedCommand<T>> {
        if input_args.len() >= rcmd.num_args as usize {
            if let Some(argt) = rcmd.arg_type {
                let mut arg_container = ArgContainer::new();
                match argt {
                    ArgType::ArgType_String => {
                        for i in 0..rcmd.num_args {
                            arg_container.args_string.push(input_args[i as usize]);
                        }
                    },
                    ArgType::ArgType_u32 => {
                        for i in 0..rcmd.num_args {
                            match input_args[i as usize].parse::<u32>() {
                                Ok(v) => arg_container.args_u32.push(v),
                                Err(e) => {
                                    // TODO: Print/return incorrect argument format and help
                                    return None
                                }
                            }
                        }
                    }
                }
                return Some(ParsedCommand::new(rcmd.cmd_id, rcmd.arg_type, Some(arg_container)));
            }
        }
        else {
            // TODO: Not enough arguments for the command. Print/return command help or use default arguments.
        }
        None
    }
}

pub struct ParsedCommand<T> {
    cmd_id: T,
    arg_type: Option<ArgType>,
    args: Option<ArgContainer>,
}

impl<T> ParsedCommand<T> {
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

    pub fn get_args_string(&self) -> &Vec<String> {
        &self.args.args_string
    }

    pub fn get_args_u32(&self) -> &Vec<u32> {
        &self.args.args_u32
    }
}

pub struct RegisteredCommand<T> {
    cmd_aliases: Vec<String>,
    cmd_id: T,
    num_args: u32,
    arg_type: Option<ArgType>,
    help_str: String,
    default_args: ArgContainer,
}

impl<T> RegisteredCommand<T> {
    pub fn new(id: T) -> RegisteredCommandBuilder<T> {
        RegisteredCommandBuilder::new(id)
    }

    pub fn is_cmd(&self, other_str: String) -> bool {
        for alias in self.cmd_aliases {
            if alias == other_str {
                return true;
            }
        }
        false
    }
}

pub struct RegisteredCommandBuilder<T> {
    cmd_aliases: Vec<String>,
    cmd_id: T,
    num_args: u32,
    arg_type: Option<ArgType>,
    help_str: String,
    default_args: ArgContainer,
}

impl<T> RegisteredCommandBuilder<T> {
    pub fn new(id: T) -> RegisteredCommandBuilder<T> {
        RegisteredCommandBuilder {
            cmd_aliases: Vec::new(),
            cmd_id: id,
            num_args: 0,
            arg_type: None,
            help_str: String::new(),
            default_args: ArgContainer::new(),
        }
    }

    pub fn add_aliases(mut self, cmd_aliases: &[&str]) -> RegisteredCommandBuilder<T> {
        self.cmd_aliases.append(&mut cmd_aliases.iter().map(|c| String::from(*c)).collect());
        self
    }

    pub fn num_args(mut self, n: u32) -> RegisteredCommandBuilder<T> {
        self.num_args = n;
        self
    }

    pub fn help_str(mut self, help: &str) -> RegisteredCommandBuilder<T> {
        self.help_str = String::from(help);
        self
    }

    pub fn help_string(mut self, help: String) -> RegisteredCommandBuilder<T> {
        self.help_str = help;
        self
    }

    pub fn default_args_string(mut self, def: Vec<String>) -> RegisteredCommandBuilder<T> {
        self.default_args.args_string = def;
        self
    }

    pub fn default_args_u32(mut self, def: Vec<u32>) -> RegisteredCommandBuilder<T> {
        self.default_args.args_u32 = def;
        self
    }

    pub fn build(self) -> Option<RegisteredCommand<T>> {
        Some(RegisteredCommand {
            cmd_aliases: self.cmd_aliases,
            cmd_id: self.cmd_id,
            num_args: self.num_args,
            arg_type: self.arg_type,
            help_str: self.help_str,
            default_args: ArgContainer::new(),
        })
    }
}