
extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::json;


#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Photo {
    url: String,
    dimensions: (usize, usize),
}


#[derive(RustcDecodable, RustcEncodable, Debug)]
struct User {
    name: String,
    post_count: usize,
    likes_burgers: bool,
    avatar: Option<Photo>,
}

fn main() {
    let user = User {
        name: "唐刚".to_string(),
        post_count: 100,
        likes_burgers: true,
        avatar: Some( Photo {
            url: "http://www.baidu.com".to_string(),
            dimensions: (160, 160),
        })
    };

    let result = json::encode(&user).unwrap();
    
    println!("{}", result);

    let obj: User = json::decode(&result).unwrap();
    
    println!("{:?}", obj);

    //println!("{:?}", json::encode(&user).unwrap());

}

