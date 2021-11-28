use widerror::*;

#[test]
fn basic() {
    let err = WidError::new().set_source(Box::new(WidError::default()));
    println!("default widerror: {}", &err);
    println!("{}", serde_json::to_string_pretty(&err).unwrap());
}
