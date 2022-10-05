use std::env;

use io_project::Config;

fn main() {
    let args = env::args();

    let config = Config::build(args).unwrap_or_else(|why| {
        eprintln!("problem parsing args: {why}");
        std::process::exit(1);
    });

    println!("cfg: {config:?}");
}
