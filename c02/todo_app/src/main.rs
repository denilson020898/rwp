mod state;
mod to_do;

use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_to_file};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = read_file(&String::from("./state.json"));
    println!("{:?}", state);

    // state.insert(title.to_string(), json!(status));
    // write_to_file("./state.json", &mut state);
}
