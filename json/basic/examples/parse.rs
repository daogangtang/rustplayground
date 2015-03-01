extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::json::Json;

fn main() {
    let data = Json::from_str("{\"foo\": 13, \"bar\": \"baz\"}").unwrap();
    println!("data: {}", data);
    println!("object? {}", data.is_object());

    let obj = data.as_object().unwrap();
    let foo = obj.get("foo").unwrap();

    println!("array? {:?}", foo.as_array());
    println!("u64? {:?}", foo.as_u64());

    for (key, value) in obj.iter() {
        println!("{}: {}", key, match *value {
            Json::U64(v) =>  format!("{} (u64)", v),
            Json::String(ref v) => format!("{} (String)", v),
            _ => format!("other")
        });
    }
}


