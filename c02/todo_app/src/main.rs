mod state;
mod processes;
mod to_do;

use serde_json::value::Value;
use serde_json::Map;
use state::read_file;
use std::env;
use to_do::to_do_factory;
use processes::process_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file("./state.json");

    let status: String;

    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = "pending".to_string();
        }
    }

    let item = to_do_factory(&status, title).expect(&status);
    process_input(item, command.to_string(), &state); 
}
