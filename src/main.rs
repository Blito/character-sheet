use std::process;

use terminal_test_1::Config;

fn main() {

    let args: Vec<String> = Vec::new();
    let config = Config::new(&args);

    if let Err(e) = terminal_test_1::run(config) {
        println!("Application Error: {}", e);

        process::exit(1);
    }
}