use actix_web::{HttpResponse, post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationCreateRequest {
    name: String,
}

#[post("/applications")]
pub async fn create_application(request: web::Json<ApplicationCreateRequest>) -> HttpResponse {
    HttpResponse::Created().body(request.name.clone())
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::{self}, test};
    use actix_web::dev::Service;

    use super::*;

    #[actix_web::test]
    async fn create_application_should_ok() {
        let app =
            test::init_service(App::new().service(create_application))
                .await;

        let req = test::TestRequest::post()
            .uri("/applications")
            .set_json(ApplicationCreateRequest {
                name: "test-application".to_owned(),
            })
            .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }
}