use std::env;
use std::process;

use minigrep::{Config,run};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}",config.get_query());
    // println!("In file {}",config.get_file_path());

    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

