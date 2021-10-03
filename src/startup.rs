use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer};
use actix_web::{HttpRequest, Responder};
use std::{io::Error, net::TcpListener};

pub fn run(listner: TcpListener) -> Result<Server, Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listner)?
    .run();
    Ok(server)
}

pub async fn index() -> &'static str {
    "Heroku Rust Api"
}

pub async fn greet(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("anonymous");
    format!("Hello {}, glad to have you onboard", name)
}
