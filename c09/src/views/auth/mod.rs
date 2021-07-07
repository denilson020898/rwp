mod login;
mod logout;

use super::path::Path;
use actix_web::web;

pub fn auth_factory(app: &mut web::ServiceConfig) {
    let base_path = Path {
        prefix: String::from("/auth"),
        backend: true
    };
    let app = app.route(
        &base_path.define(String::from("/login")),
        web::post().to(login::login),
    );
    app.route(
        &base_path.define(String::from("/logout")),
        web::post().to(logout::logout),
    );
}