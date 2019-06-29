use actions::{create_subscription, remove_subscription};
use actix_web::{http, web, HttpRequest, HttpResponse};
use askama::Template;
use url::form_urlencoded;
use utils::{validate_parsed_data, AppState};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexView;

pub fn index(state: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(IndexView.render().unwrap())
}

pub fn hub(state: web::Data<AppState>, _req: HttpRequest, params: String) -> HttpResponse {
    let log = &state.log;
    info!(log, "Received Request");
    debug!(log, "Content: {}", params);
    let parsed_data = form_urlencoded::parse(params.as_bytes());

    if !validate_parsed_data(parsed_data) {
        return HttpResponse::Ok()
            .status(http::StatusCode::from_u16(400).unwrap())
            .finish();
    }

    HttpResponse::Ok().body("Hello World!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix::{SyncArbiter, System};
    use actix_web::{http, test, web};
    use diesel::prelude::*;
    use utils::{setup_logging, DbExecutor};

    #[test]
    fn test_index() {
        let _sys = System::new("rusty-hub-test");
        let addr = SyncArbiter::start(1, || {
            DbExecutor(SqliteConnection::establish("test.db").unwrap())
        });

        let data = web::Data::new(AppState {
            log: setup_logging(),
            db: addr.clone(),
        });

        let resp = index(data, test::TestRequest::get().to_http_request());
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_hub() {
        let _sys = System::new("rusty-hub-test");
        let addr = SyncArbiter::start(1, || {
            DbExecutor(SqliteConnection::establish("test.db").unwrap())
        });

        let data = web::Data::new(AppState {
            log: setup_logging(),
            db: addr.clone(),
        });

        let resp = hub(
            data,
            test::TestRequest::post().to_http_request(),
            "key=value".to_string(),
        );
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}
