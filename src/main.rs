use std::net::TcpListener;

use heroku_rust_api::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("ACTIX_PORT").unwrap_or("8000".into());
    let address = format!("127.0.0.1:{}", port);
    println!("Starting web api @ {}", &address);
    let listener = TcpListener::bind(&address)?;
    run(listener)?.await
}
