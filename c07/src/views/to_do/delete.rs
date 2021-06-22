use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};
use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_ser::to_do_items::ToDoItem;
use crate::models::item::item::Item;
use crate::schema::to_do;

pub async fn delete(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_ref: String = to_do_item.title.clone();
    let conn = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&conn)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&conn);
    HttpResponse::Ok().json(return_state())
}
