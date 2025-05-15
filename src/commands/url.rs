use crate::output::CommandResult;
use urlencoding::{decode, encode};

pub fn urlenc(text: &str) -> CommandResult<String> {
    let encoded = encode(text).to_string();

    CommandResult {
        status: true,
        value: encoded,
    }
}

pub fn urldec(text: &str) -> CommandResult<String> {
    match decode(text) {
        Ok(result_str) => CommandResult {
            status: true,
            value: result_str.to_string(),
        },
        Err(msg) => CommandResult {
            status: false,
            value: msg.to_string(),
        },
    }
}
