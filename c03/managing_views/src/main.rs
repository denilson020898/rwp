use actix_web::{App, HttpServer};
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("localhost:8000")?
    .run()
    .await
}
