use crate::output::CommandResult;
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn help() -> CommandResult<String> {
    CommandResult {
        status: true,
        value: String::from("Ready-to-go toolbox"),
    }
}

pub fn version() -> CommandResult<String> {
    CommandResult {
        status: true,
        value: String::from(VERSION),
    }
}

pub fn unknown() -> CommandResult<String> {
    CommandResult {
        status: true,
        value: String::from("Unknown command"),
    }
}
