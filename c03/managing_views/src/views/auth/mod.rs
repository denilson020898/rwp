mod login;
mod logout;

use super::path::Path;
use actix_web::web;

pub fn auth_factory(app: &mut web::ServiceConfig, logout: bool) {
    let base_path = Path {
        prefix: String::from("/auth"),
    };
    let app = app.route(
        &base_path.define(String::from("/login")),
        web::get().to(login::login),
    );

    if logout {
        app.route(
            &base_path.define(String::from("/logout")),
            web::get().to(logout::logout),
        );
    }
}
