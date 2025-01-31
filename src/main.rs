mod modes;
mod manifest;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 { println!("Please, provide mode") }

    match args[1].as_str() {
        "push" => {
            modes::push::enter_push(&args);
        },
        "pull" => {
            modes::pull::enter_pull(&args);
        },
        "init" => {
            modes::init::enter_init(&args);
        }
        _ => {
            println!("Error: unknown mode <{}>", args[1])
        }
    }
}
