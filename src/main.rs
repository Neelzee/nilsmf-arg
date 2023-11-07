use actix_web::http::header::Header;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web, http};
use std::fs::File;
use std::io::{self, Read};
use actix_cors::Cors;
use walkdir::WalkDir;

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

#[get("/images/{file}")]
async fn get_image(path: web::Path<String>) -> impl Responder {
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

#[get("/article/projects")]
async fn get_projects() -> impl Responder {
    let res: Result<Vec<String>, io::Error> = async {
        let mut file_contents: Vec<String> = Vec::new();

        for entry in WalkDir::new("./markdown/projects").into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path();

                let mut file = File::open(file_path)?;
                let mut file_content = String::new();
                file.read_to_string(&mut file_content)?;

                file_contents.push(file_content);
            }
        }

        Ok(file_contents)
    }.await;
    
    if let Ok(con) = res {

        let json_data = web::Json(con);

        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Content-Type", "text/markdown"))
            .json(json_data);
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
            .service(get_projects)
            .service(get_file)
            .service(get_image)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
