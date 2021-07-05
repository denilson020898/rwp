use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::{json_ser::to_do_items::ToDoItems, to_do::to_do_factory};
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;

pub fn return_state(user_id: &i32) -> ToDoItems {
    let conn = establish_connection();
    let items = to_do::table
        .order(to_do::columns::id.asc())
        .filter(to_do::columns::user_id.eq(&user_id))
        .load::<Item>(&conn)
        .unwrap();
    let mut array_buffer = Vec::new();
    for item in items {
        let item = to_do_factory(&item.status, &item.title).unwrap();
        array_buffer.push(item);
    }
    ToDoItems::new(array_buffer)
}
