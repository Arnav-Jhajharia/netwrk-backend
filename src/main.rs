mod streams;

use actix_web::{  post, App, HttpRequest, HttpResponse, HttpServer, Responder};
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod};

/// Post to "echo" something, and it will parse the headers
#[post("/echo")]
async fn echo(req: HttpRequest, req_body:String) -> impl Responder {
    println!("the following headers were sent");
    for (name, value) in req.headers().iter() {
        println!("{}: {}", name , value.to_str().unwrap_or("invalid utf-8 formatting") );
    };
    HttpResponse::Ok().body(req_body)
}



#[actix_web::get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    ssl_builder.set_private_key_file("dev/self-signed-ssl/key.pem", SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file("dev/self-signed-ssl/cert.pem")?;
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(index)
            .service(streams::streams)
    })
        .workers(std::thread::available_parallelism().unwrap().get())
        .bind_openssl(("127.0.0.1", 8443), ssl_builder )?
        .run()
        .await
}