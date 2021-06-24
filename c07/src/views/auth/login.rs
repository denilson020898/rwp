use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::database::establish_connection;
use crate::models::user::user::User;
use crate::json_ser::login::Login;
use crate::schema::users;
use crate::auth::jwt::JwtToken;

pub async fn login(credentials: web::Json<login>) -> Htt {
    format!("Login view")
}

