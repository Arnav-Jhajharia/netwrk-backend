
mod streams;
mod manager;

use std::num::NonZeroUsize;
use tokio::sync::RwLock;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use lru::LruCache;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::manager::MAX_CACHE_SIZE;

#[actix_web::get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Logging middleware
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();


    // certificate set up for https://
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    ssl_builder.set_private_key_file("dev/self-signed-ssl/key.pem", SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file("dev/self-signed-ssl/cert.pem")?;

    // state manager with lru-cache (for now)
    let state_manager = web::Data::new(manager::StateManager{
        lru_cache : RwLock::new(LruCache::new(NonZeroUsize::new(MAX_CACHE_SIZE).unwrap()))
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state_manager.clone())
            .wrap(Logger::new(r#"%a %{r}a "%r" %s %b "%{Referer}i" %Dms"#))
            .service(streams::streams)
            .service(index)
    })
        .workers(4)
        .bind_openssl(("127.0.0.1", 8443), ssl_builder )?
        .run()
        .await
}