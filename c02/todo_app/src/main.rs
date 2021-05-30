mod to_do;

use to_do::ItemTypes;
use to_do::to_do_factory;

fn main() {
    let to_do_item = to_do_factory("done", "masturbate");

    match to_do_item.unwrap() {
        ItemTypes::Done(item) => println!("it is a done item with title: {}", item.super_struct.title),
        ItemTypes::Pending(item) => println!("it is a pending item with title: {}", item.super_struct.title)
    }
}

