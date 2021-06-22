use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::json_ser::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;

pub async fn create(new_user: web::Json<NewUserSchema>) -> HttpResponse {
    let conn = establish_connection();
    let name = new_user.name.clone();
    let email = new_user.email.clone();
    let password = new_user.password.clone();

    let new_user = NewUser::new(name, email, password);

    let insert_result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn);
    match insert_result {
        Ok(_) => HttpResponse::Created().await.unwrap(),
        Err(_) => HttpResponse::Conflict().await.unwrap(),
    }
}
