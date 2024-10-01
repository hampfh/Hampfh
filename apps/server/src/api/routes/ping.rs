use actix_web::get;

#[get("/api/ping")]
pub(routes) async fn get_api_ping() -> actix_web::Result<String> {
    return Ok("pong".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_ping_route() {
        let app = test::init_service(App::new().service(get_api_ping)).await;
        let req = test::TestRequest::get().uri("/api/ping").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "pong");
    }
}
