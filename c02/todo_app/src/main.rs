mod to_do;

use to_do::ItemTypes;
use to_do::to_do_factory;

use crate::to_do::structs::traits::create::Create;

fn main() {
    let to_do_item = to_do_factory("pending", "masturbate");

    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!("it is a pending item with title: {}", item.super_struct.title)
    }
}

