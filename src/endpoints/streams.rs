use crate::manager::StateManager;
use actix_web::http::header::HeaderMap;
use actix_web::{HttpRequest, HttpResponse, web};
use apache_avro::Reader;
use std::time::SystemTime;

async fn authenticate(state: web::Data<StateManager>, headers: &HeaderMap) -> bool {
    if headers.get("Authorization").is_some() {
        let auth = headers.get("Authorization").unwrap().to_str().unwrap();
        let from_cache = state.cache_get(auth.into()).await;
        if from_cache {
            return true;
        }
        state
            .cache_upsert(auth.parse().unwrap(), SystemTime::now())
            .await;
        return true;
    }
    false
}
pub async fn stream(
    state: web::Data<StateManager>,
    body: web::Bytes,
    req: HttpRequest,
) -> HttpResponse {
    let authenticated = authenticate(state, req.headers()).await;

    if authenticated {
        let bytes_vec = body.to_vec();
        let result = Reader::new(&bytes_vec[..]);

        return match result {
            Ok(_res) => HttpResponse::Ok().body("Stream was read"),
            Err(_) => HttpResponse::InternalServerError()
                .body("Server encountered an error while processing stream"),
        };
    };
    HttpResponse::Unauthorized().body("Unauthorized access detected.")
}
