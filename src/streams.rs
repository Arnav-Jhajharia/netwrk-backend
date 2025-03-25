use std::time::SystemTime;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::http::header::HeaderMap;
use actix_web::web::BytesMut;
use apache_avro::Reader;
use futures_util::StreamExt as _;
use crate::manager::StateManager;

async fn authenticate(state: web::Data<StateManager>, headers: &HeaderMap) -> bool {
    if headers.get("Authentication").is_some() {
        let auth = headers.get("Authentication").unwrap().to_str().unwrap();
        let from_cache = state.cache_get(auth.into()).await ;
        if from_cache {
            return true;
        }
        state.cache_upsert(auth.parse().unwrap(), SystemTime::now()).await;
        return true;
    }
    false
}

/// Handles Streams
#[actix_web::post("/streams")]
async fn streams (state: web::Data<StateManager>, mut payload : web::Payload, req : HttpRequest) -> HttpResponse {

    let authenticated = authenticate(state, req.headers()).await;

    if authenticated {
        // get the body from the stream
        let mut body = BytesMut::new();
        while let Some(chunk) = payload.next().await {
            let chunk = chunk.unwrap();
            body.extend_from_slice(&chunk);
        }
        let bytes_vec = body.to_vec();
        let result = Reader::new(&bytes_vec[..]);

        return match result {
            Ok(res) => {
                let mut i = 0;
                for _value in res {
                    i += 1;
                }
                return if  i > 0 {
                    HttpResponse::Ok().body("Stream was read")
                }else {
                    HttpResponse::Ok().body("Stream was read, but i failed")
                }
            },
            Err(_) => HttpResponse::InternalServerError().body("Server encountered an error while processing stream"),
        }
    };
    HttpResponse::Unauthorized().body("Unauthorized access detected.")
}