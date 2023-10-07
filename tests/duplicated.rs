//! Test for duplicated emojis. Make sure to run this before opening your PR!

use std::collections::HashSet;
use cat_ascii_faces::Cats;

#[test]
fn main() {
    let mut uniq = HashSet::new();
    let cats = Cats::all();
    let x = cats.iter().all(|&x| uniq.insert(x));
    if !x {
        panic!("Cat face duplicated: \"{}\"", cats[uniq.len()])
    }
}