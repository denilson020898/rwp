use crate::diesel;
use diesel::prelude::*;

use super::utils::return_state;
use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_ser::to_do_items::ToDoItem;
use crate::schema::to_do;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_ref: String = to_do_item.title.clone();
    let conn = establish_connection();
    let results = to_do::table.filter(to_do::columns::title.eq(title_ref));
    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&conn);
    HttpResponse::Ok().json(return_state())
}
