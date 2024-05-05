use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    // report error call process exit if build() fails and unwrap explodes unexpectedly
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // report error and call process exit IF Err type is returned unexpectedly
    if let Err(e) = minigrep::run(config) {
        println!("Problem running application: {e}");
        process::exit(1);
    }
}
