use crate::output::CommandResult;

pub fn help() -> CommandResult<String> {
    CommandResult {
        status: true,
        value: String::from("Ready-to-go toolbox"),
    }
}

pub fn version() -> CommandResult<String> {
    CommandResult {
        status: true,
        value: String::from("0.1.0"),
    }
}

pub fn unknown() -> CommandResult<String> {
    CommandResult {
        status: true,
        value: String::from("Unknown command"),
    }
}
