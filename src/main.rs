// mgrep - Mini grep version in rust

use std::env;

use mgrep::{run, Query};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: Query = Query::new(&args);
    run(&query);
}
