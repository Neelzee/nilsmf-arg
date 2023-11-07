use actix_web::{get, HttpResponse, Responder, web};


#[get("/images/{file}")]
pub async fn get_image(path: web::Path<String>) -> impl Responder {
    let file_path = String::from("./images/".to_string() + &path.into_inner());

    if let Ok(res) = std::fs::read(file_path) {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .content_type("image/png")
            .body(res);
    }
    
    HttpResponse::NotFound().insert_header(("Access-Control-Allow-Origin", "*")).body("File not found")
}