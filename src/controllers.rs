use actix_web::{HttpRequest, HttpResponse};
use askama::{Template};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexView;


pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        IndexView.render().unwrap()
    )
}

pub fn hub(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[test]
    fn test_index() {
        let resp = index(test::TestRequest::default().finish());
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_hub() {
        let resp = hub(test::TestRequest::default().finish());
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
