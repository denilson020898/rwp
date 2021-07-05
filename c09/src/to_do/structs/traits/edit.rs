use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        println!("\n\n{} is being set to done\n\n", title);
    }
    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("pending")));
        println!("\n\n{} is being set to pending\n\n", title);
    }
}
