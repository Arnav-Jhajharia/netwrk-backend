use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::web::BytesMut;
use futures_util::StreamExt as _;

/// Handles Streams
#[actix_web::post("/streams")]
async fn streams (mut payload : web::Payload, req : HttpRequest) -> impl Responder {

    println!("Headers for this request are");
    for (name, value) in req.headers().iter() {
        println!("{}, {}" , name, value.to_str().unwrap_or("invalid utf-8 formatting") );
    }

    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        body.extend_from_slice(&chunk);
    }

    println!("body: {:?}", body);

    HttpResponse::Ok().body(body)
}