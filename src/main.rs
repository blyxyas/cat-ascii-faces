#![allow(non_camel_case_types)]
#![feature(let_chains)]

use std::env;
use cat_ascii_faces::Cats;

fn main() {
    let maybe_arg = env::args().nth(1);
    if let Some(arg) = maybe_arg && (arg == "-a" || arg == "--all") {
        let cats = Cats::all();
        for cat in cats {
            println!("{}", cat);
        }
    } else {
        let mut cats = Cats::new();
        println!("{}", cats.cat());
    }
}
