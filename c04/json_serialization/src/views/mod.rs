mod auth;
mod path;
mod to_do;

use actix_web::web;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    to_do::item_factory(app);
}
