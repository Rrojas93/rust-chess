
pub enum Command {
    Move(Option<String>),
    Undo(Option<u32>),
    Redo(Option<u32>),
    Reset,
    Save(Option<String>),
    Quit,
}

pub fn parse_command(commands: Vec<RegisteredCommand>, user_input: String) -> Option<Command> {
    let mut iter = user_input.trim().split_whitespace();
    if let Some(input_cmd) = iter.next() {
        for registered_cmd in commands {
            if registered_cmd.compare(input_cmd) {
                let cmd_type = registered_cmd.get_cmd_type();
                let arg = iter.next();
                match cmd_type {
                    Command::Move(_) => {
                        if let Some(a) = arg {
                            return Some(Command::Move(Some(String::from(a))));
                        }
                    },
                    Command::Undo(_) => {
                        if let Some(a) = arg {
                            if let Ok(num) = a.parse::<u32>() {
                                return Some(Command::Undo(Some(num)));
                            }
                        }
                    },
                    Command::Redo(_) => {
                        if let Some(a) = arg {
                            if let Ok(num) = a.parse::<u32>() {
                                return Some(Command::Redo(Some(num)));
                            }
                        }
                    },
                    Command::Save(_) => {
                        if let Some(a) = arg {
                            return Some(Command::Save(Some(String::from(a))));
                        }
                    },
                    Command::Reset => return Some(Command::Reset),
                    Command::Quit => return Some(Command::Quit),
                }
            }
        }
    }
    None
}

pub struct RegisteredCommand {
    cmd_str: String,
    cmd_type: Command,
}

impl RegisteredCommand {
    pub fn new(cmd_str: String, cmd_type: Command) -> RegisteredCommand {
        RegisteredCommand { cmd_str, cmd_type }
    }

    pub fn compare(&self, other_str: &str) -> bool {
        self.cmd_str == other_str
    }

    pub fn get_cmd_type(&self) -> &Command {
        &self.cmd_type
    }
}
