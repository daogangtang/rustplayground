extern crate "rustc-serialize" as rustc_serialize;
use rustc_serialize::json;

fn main() {
    println!("{:?}", json::encode(&42).unwrap());
    println!("{:?}", json::encode(&vec!["to", "be", "or", "not", "to", "be"]).unwrap());
    println!("{:?}", json::encode(&Some(true)).unwrap());
    

}
