use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

    //unlike Config::build() run(config) doesnt return any value so instead of using unwrap_or_else() we are if let.
    //run(config) returns just () for the Ok variant of Result<>
    //The bodies of the if let and the unwrap_or_else functions are the same in both cases: we print the error and exit.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
