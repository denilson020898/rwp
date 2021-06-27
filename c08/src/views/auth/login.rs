use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::models::user::user::User;
use crate::json_ser::login::Login;
use crate::schema::users;
use crate::auth::jwt::JwtToken;

pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let username = credentials.username.clone();
    let password = credentials.password.clone();

    let conn = establish_connection();
    let users = users::table
        .filter(users::columns::username.eq(username.as_str()))
        .load::<User>(&conn).unwrap();
    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }

    match users[0].clone().verify(password) {
        true => {
            let token = JwtToken::encode(users[0].clone().id);
            HttpResponse::Ok().header("token", token).await.unwrap()
        }
        false => {
            HttpResponse::Unauthorized().await.unwrap()
        }
    }
}

