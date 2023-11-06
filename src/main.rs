use actix_web::http::header::Header;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web, http};
use std::fs::File;
use std::io::{self, Read};
use actix_cors::Cors;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/article/{file}")]
async fn get_file(path: web::Path<String>) -> impl Responder {
    let file_path = path.into_inner();

    let res: Result<String, io::Error> = async {
        let fp = String::from("./markdown/".to_string() + &file_path);
        let mut file = File::open(fp)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(contents)
    }.await;
    
    if let Ok(con) = res {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Content-Type", "text/markdown"))
            .body(con);
    }

    HttpResponse::NotFound().insert_header(("Access-Control-Allow-Origin", "*")).body("File not found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            // fn service(f: impl Fn() -> impl Responder)
            .service(hello)
            .service(echo)
            .service(get_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
