use actix_web::{HttpRequest, Responder};

pub fn index(_req: &HttpRequest) -> impl Responder {
    "Hello world!"
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[test]
    fn test_index() {
        let resp = test::TestRequest::default().run(&index).unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
