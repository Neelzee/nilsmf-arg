use actix_web::{get, HttpResponse, Responder, web};


#[get("/images/{file}")]
pub async fn get_image(path: web::Path<String>) -> impl Responder {
    let file_path = String::from("./images/".to_string() + &path.into_inner());

    if let Ok(res) = web::block(|| std::fs::read(file_path)).await {
        if let Ok(con) = res {
            return HttpResponse::Ok()
                .insert_header(("Access-Control-Allow-Origin", "*"))
                .content_type("image/png")
                .body(con);
        }
    }
    HttpResponse::NotFound().insert_header(("Access-Control-Allow-Origin", "*")).body("File not found")
}