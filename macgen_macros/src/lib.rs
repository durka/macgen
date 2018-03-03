#![feature(proc_macro)]

#[macro_use] extern crate lazy_static;
extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::{Ident, LitInt};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    static ref INSTANCES: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
}

#[proc_macro]
pub fn macgen(_input: TokenStream) -> TokenStream {
    let path = Path::new(env!("OUT_DIR")).join("macgen.rs");
    let path_str = path.to_str().unwrap();

    let mut file = File::create(&path).unwrap();
    for instance in INSTANCES.lock().unwrap().iter() {
        let name = Ident::from(format!("name_{}", instance));
        let string = format!("I am number {}!", instance);
        writeln!(file, "{}", quote! {
            pub fn #name() {
                println!(#string);
            }
        }).unwrap();
    }

    let expanded = quote! {
        #[path=#path_str] mod macgen;
        use macgen::*;
    };

    expanded.into()
}

#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
    let value = syn::parse::<LitInt>(input).unwrap().value();

    INSTANCES.lock().unwrap().insert(value);

    let name = Ident::from(format!("name_{}", value));
    let expanded = quote! {
        #name()
    };

    expanded.into()
}

