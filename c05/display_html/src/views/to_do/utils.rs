use crate::{json_ser::to_do_items::ToDoItems, state::read_file, to_do::{ItemTypes, to_do_factory}};
use serde_json::{self, Map, Value};

pub fn return_state() -> ToDoItems {
    let state: Map<String, Value> = read_file(&String::from("./state.json"));
    let mut array_buffer = Vec::new();

    for (key, value) in state {
        let item_type: String = String::from(value.as_str().unwrap());
        let item: ItemTypes = to_do_factory(&item_type, &key).unwrap();
        array_buffer.push(item);
    }
    ToDoItems::new(array_buffer)
}
