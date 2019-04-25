use actix_web::{HttpRequest, HttpResponse};
use askama::Template;
use utils::AppState;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexView;

pub fn index(_req: HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(IndexView.render().unwrap())
}

pub fn hub(_req: HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::actix::{SyncArbiter, System};
    use actix_web::{http, test};
    use diesel::prelude::*;
    use utils::{setup_logging, DbExecutor};

    #[test]
    fn test_index() {
        let _sys = System::new("rusty-hub-test");
        let addr = SyncArbiter::start(1, || {
            DbExecutor(SqliteConnection::establish("test.db").unwrap())
        });

        let resp = index(
            test::TestRequest::with_state(AppState {
                log: setup_logging(),
                db: addr.clone(),
            })
            .finish(),
        );
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_hub() {
        let _sys = System::new("rusty-hub-test");
        let addr = SyncArbiter::start(1, || {
            DbExecutor(SqliteConnection::establish("test.db").unwrap())
        });

        let resp = hub(test::TestRequest::with_state(AppState {
            log: setup_logging(),
            db: addr.clone(),
        })
        .finish());
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
