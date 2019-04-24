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
    use actix_web::{http, test};
    use utils::setup_logging;

    #[test]
    fn test_index() {
        let resp = index(
            test::TestRequest::with_state(AppState {
                log: setup_logging(),
            })
            .finish(),
        );
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_hub() {
        let resp = hub(test::TestRequest::with_state(AppState {
            log: setup_logging(),
        })
        .finish());
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
