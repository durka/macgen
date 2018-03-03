#![feature(proc_macro)]

extern crate macgen_macros;
use macgen_macros::*;

fn main() {
    foo!(1);
    foo!(2);
    foo!(1);
}

macgen!();

