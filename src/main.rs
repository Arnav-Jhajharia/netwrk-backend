use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod};
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    ssl_builder.set_private_key_file("./key.pem", SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file("./cert.pem")?;
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .workers(10)
        .bind_openssl(("127.0.0.1", 8443), ssl_builder )?
        .run()
        .await
}