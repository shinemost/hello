use hello::Config;
use std::env;
use std::process;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    print!("Searching for {} ", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = hello::run(config) {
        // --snip--
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
