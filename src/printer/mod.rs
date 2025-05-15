use crate::output::CommandResult;

pub fn print_result<Value: std::fmt::Display>(result: CommandResult<Value>) {
    if result.status {
        println!("{}", result.value);
    } else {
        println!("Error: {}", result.value);
    }
}
