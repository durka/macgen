extern crate foreman;

use std::env;

fn main() {
    foreman::env_var("OUT_DIR", &env::var("OUT_DIR").unwrap());
}

