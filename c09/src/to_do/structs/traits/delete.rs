use serde_json::value::Value;
use serde_json::Map;

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title);
        println!("\n\n{} is being deleted\n\n", title);
    }
}
