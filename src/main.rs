extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

// mid point between the program
// and user interaction
fn main() {
    let args: Vec<String> = env::args().collect();
    
    // gets the arguments from the user and
    // processes them to be able to used by the program
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // program logic seperated
    if let Err(e) = minigrep::run(config) {
        eprintln!("Applicaion error: {}", e);
        process::exit(1);
    }
}
