use std::net::TcpListener;

use heroku_rust_api::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(&address)?;
    run(listener)?.await
}
