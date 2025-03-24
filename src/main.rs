
mod streams;

use actix_web::{ App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};


#[actix_web::get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    ssl_builder.set_private_key_file("dev/self-signed-ssl/key.pem", SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file("dev/self-signed-ssl/cert.pem")?;
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(streams::streams)
            .service(index)
    })
        .workers(std::thread::available_parallelism().unwrap().get())
        .bind_openssl(("127.0.0.1", 8443), ssl_builder )?
        .run()
        .await
}