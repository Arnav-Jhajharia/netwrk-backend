use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::BytesMut;
use futures_util::StreamExt as _;

/// Handles Streams
#[actix_web::post("/streams")]
async fn streams (mut payload : web::Payload, _req : HttpRequest) -> HttpResponse {

    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        body.extend_from_slice(&chunk);
    }
    HttpResponse::Ok().body(body)
}