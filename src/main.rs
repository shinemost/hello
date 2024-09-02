use hello::Config;
use std::env;
use std::process;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = hello::run(config) {
        // --snip--
        println!("Application error: {e}");
        process::exit(1);
    }
}
