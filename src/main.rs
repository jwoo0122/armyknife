mod commands;
mod output;
mod printer;

use std::env;

use printer::print_result;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No command");
        return;
    }

    let command = args[1].clone();
    let command_result = commands::handle_command(&command, &args);
    print_result(command_result);
}
