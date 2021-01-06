use rss_management::initializer::Initializer;

use std::fs;

fn main() {
    println!("Hello, world!");
    let _i = Initializer::new_from_str("/tmp/blbl", "/tmp/blbl2");
    _i.initialize();
}
