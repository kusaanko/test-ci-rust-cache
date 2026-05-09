use serde::Serialize;

fn main() {
    println!("Hello, world!");
    log::info!("This is an info message.");
    log::warn!("This is a warning message.");
}

#[derive(Serialize)]
struct MyStruct {
    field1: String,
    field2: i32,
}
