mod auth;
mod path;
mod to_do;
mod app;
pub mod token;

use actix_web::web;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    to_do::item_factory(app);
    app::app_factory(app);
}
