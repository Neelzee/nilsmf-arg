use actix_web::{get, web, HttpResponse, Responder};

use crate::utils::{consts::PATH, db::fetch_blog, funcs::get_file_content};

#[get("/articles/{file}")]
pub async fn get_article(path: web::Path<String>) -> impl Responder {
    let file_path = path.into_inner();

    let res = get_file_content(String::from(format!(
        "{}{}{}",
        PATH, "markdown/articles/", &file_path
    )));

    if let Ok(con) = res {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Content-Type", "text/markdown"))
            .body(con);
    }

    HttpResponse::NotFound()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body(String::from("File not found: ".to_string() + &file_path))
}

#[get("/blogs/{file}")]
pub async fn get_blog(path: web::Path<String>) -> impl Responder {
    let blog_id = path.into_inner();

    fetch_blog(blog_id);

    return HttpResponse::NotFound()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body(format!("Path not found: {}", blog_id));
}
