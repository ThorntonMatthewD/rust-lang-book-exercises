use std::env;
use std::process;

use minigrep::Config;

/*
 * "It’s better to have a working program that’s a bit inefficient
 * than to try to hyperoptimize code on your first pass."
 */

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem encountered while parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Something broke: {e}");
        process::exit(1);
    };
}
