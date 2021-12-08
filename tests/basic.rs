use std::string::String;

use widerror::*;

#[test]
fn basic() {
    let err = WidError::new(123456789, Message::Default(String::from("this is message"))).with_source(WidError::default());
    println!("default widerror: {}", &err);
    println!("{}", serde_json::to_string_pretty(&err).unwrap());
}
