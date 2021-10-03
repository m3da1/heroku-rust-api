use std::net::TcpListener;

use heroku_rust_api::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let address = format!("0.0.0.0:{}", port);
    println!("Starting web api @ {}", &address);
    let listener = TcpListener::bind(&address)?;
    run(listener)?.await
}
