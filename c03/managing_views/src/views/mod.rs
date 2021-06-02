mod auth;
mod path;

use std::env;

use actix_web::web;

pub fn views_factory(app: &mut web::ServiceConfig) {
    let args: Vec<String> = env::args().collect();
    let param = &args[args.len() - 1];
    if param.as_str() == "cancel_logout" {
        println!("logout view is not being configured");
        auth::auth_factory(app, false);
    } else {
        auth::auth_factory(app, true);
    }
}
