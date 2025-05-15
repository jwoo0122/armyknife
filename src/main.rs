mod commands;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_cloned = args.clone();

    match args_cloned[args_cloned.len() - 1].as_str() {
        "help" => commands::help(),
        "version" => commands::version(),
        _ => commands::unknown(),
    }
}
