use actix_web::{get, HttpResponse, Responder, web};

use crate::utils::funcs::get_file_content;

#[get("/articles/{file}")]
pub async fn get_article(path: web::Path<String>) -> impl Responder {
    let file_path = path.into_inner();

    let res = get_file_content(String::from("./markdown/projects/".to_string() + &file_path));
    
    if let Ok(con) = res {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Content-Type", "text/markdown"))
            .body(con);
    }

    HttpResponse::NotFound().insert_header(("Access-Control-Allow-Origin", "*")).body(String::from("File not found: ".to_string() + &file_path))
}