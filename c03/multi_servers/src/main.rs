use actix_web::{web, App, HttpServer, Responder};
use futures::future;

async fn utils_one() -> impl Responder {
    "utils one bro\n"
}

async fn health() -> impl Responder {
    "better\n"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let s1 = HttpServer::new(move || {
        App::new().service(web::scope("/utils").route("/one", web::get().to(utils_one)))
    })
    .bind("localhost:6969")?
    .run();

    let s2 = HttpServer::new(move || {
        App::new().service(web::resource("/health").route(web::get().to(health)))
    })
    .bind("localhost:9696")?
    .run();
    future::try_join(s1, s2).await?;

    Ok(())
}
