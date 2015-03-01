#![feature(io, fs)]
use std::fs::File;
use std::io::{Write, Read};

fn main() {
    let mut f = File::create("foo.txt").unwrap();
    f.write_all(b"Hello world!").unwrap();

    let mut f = File::open("foo.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    assert_eq!(s, "Hello world!");
}
