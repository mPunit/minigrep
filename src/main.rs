//use env_logger;
//use log::{debug, info};
use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
    //env_logger::init();

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
