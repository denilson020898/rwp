#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_service::Service;
use actix_web::{App, HttpResponse, HttpServer};
use futures::future::{ok, Either};
mod auth;
mod database;
mod json_ser;
mod models;
mod processes;
mod schema;
mod state;
mod to_do;
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                let passed: bool;
                if *&req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => {
                            passed = true;
                        }
                        Err(_msg) => {
                            passed = false;
                        }
                    }
                } else {
                    passed = true;
                }
                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => Either::Right(ok(
                        req.into_response(HttpResponse::Unauthorized().finish().into_body())
                    )),
                };
                end_result
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
