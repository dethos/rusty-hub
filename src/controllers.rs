use crate::actions::{handle_publication, handle_subscription};
use crate::utils::{validate_parsed_data, AppState};
use actix_web::{http, web, HttpRequest, HttpResponse};
use askama::Template;
use std::collections::HashMap;
use url::form_urlencoded;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexView;

pub fn index(_state: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(IndexView.render().unwrap())
}

pub fn hub(state: web::Data<AppState>, _req: HttpRequest, params: String) -> HttpResponse {
    let log = &state.log;
    let db = &state.db;
    info!(log, "Received Request. Content: {}", params);

    let parsed_data = form_urlencoded::parse(params.as_bytes());
    let mut parameters = HashMap::new();
    for (key, value) in parsed_data {
        parameters.insert(key.to_string(), value.to_string());
    }

    match validate_parsed_data(&parameters) {
        Ok(_) => debug!(log, "Valid request."),
        Err(reason) => {
            return HttpResponse::Ok()
                .status(http::StatusCode::BAD_REQUEST)
                .content_type("text/plain")
                .body(reason)
        }
    }

    if parameters.get("hub.mode").expect("Mode not provided") == &"publish" {
        handle_publication(db, &parameters);
    } else {
        handle_subscription(db, &parameters);
    }
    return HttpResponse::Ok()
        .status(http::StatusCode::ACCEPTED)
        .finish();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::setup_logging;
    use actix::System;
    use actix_web::{http, test, web};
    use diesel::prelude::*;
    use diesel::r2d2::{self, ConnectionManager};

    #[test]
    fn test_index() {
        let _sys = System::new("rusty-hub-test");
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let data = web::Data::new(AppState {
            log: setup_logging(),
            db: pool.clone(),
        });

        let resp = index(data, test::TestRequest::get().to_http_request());
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_hub_no_parameters() {
        let _sys = System::new("rusty-hub-test");
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let data = web::Data::new(AppState {
            log: setup_logging(),
            db: pool.clone(),
        });

        let resp = hub(
            data,
            test::TestRequest::post().to_http_request(),
            "key=value".to_string(),
        );
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_hub_invalid_callback() {
        let _sys = System::new("rusty-hub-test");
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let data = web::Data::new(AppState {
            log: setup_logging(),
            db: pool.clone(),
        });

        let resp = hub(
            data,
            test::TestRequest::post().to_http_request(),
            "hub.mode=subscribe&hub.callback=none&hub.topic=http://example.com".to_string(),
        );
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_hub_invalid_topic() {
        let _sys = System::new("rusty-hub-test");
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let data = web::Data::new(AppState {
            log: setup_logging(),
            db: pool.clone(),
        });

        let resp = hub(
            data,
            test::TestRequest::post().to_http_request(),
            "hub.mode=subscribe&hub.callback=http://example.com&hub.topic=none".to_string(),
        );
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_hub_invalid_mode() {
        let _sys = System::new("rusty-hub-test");
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let data = web::Data::new(AppState {
            log: setup_logging(),
            db: pool.clone(),
        });

        let resp = hub(
            data,
            test::TestRequest::post().to_http_request(),
            "hub.mode=none&hub.callback=http://example.com&hub.topic=http://other.com".to_string(),
        );
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_hub_subscribe_success() {
        let _sys = System::new("rusty-hub-test");
        let manager = ConnectionManager::<SqliteConnection>::new("test.db");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let data = web::Data::new(AppState {
            log: setup_logging(),
            db: pool.clone(),
        });

        let resp = hub(
            data,
            test::TestRequest::post().to_http_request(),
            "hub.mode=subscribe&hub.callback=http://example.com&hub.topic=http://other.com"
                .to_string(),
        );
        assert_eq!(resp.status(), http::StatusCode::ACCEPTED);
    }
}
