use actix_web::{get, Responder, web};

#[get("/{id}/{name}/index.html")]
pub async fn index(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{}", name, id)
}
