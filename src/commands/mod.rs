use crate::output;

mod git;
mod miscellaneous;
mod url;

pub fn handle_command(command: &str, args: &[String]) -> output::CommandResult<String> {
    let second_arg = &args[args.len() - 1];

    match command {
        "wip" => git::commit(second_arg),
        "urlenc" => url::urlenc(second_arg),
        "urldec" => url::urldec(second_arg),
        "help" => miscellaneous::help(),
        "version" => miscellaneous::version(),
        _ => miscellaneous::unknown(),
    }
}
