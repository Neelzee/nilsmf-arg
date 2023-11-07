mod api;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web, http};
use api::images::get_image;
use api::projects::{get_project, get_projects};
use std::fs::File;
use std::io::{self, Read};
use actix_cors::Cors;
use walkdir::WalkDir;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            // fn service(f: impl Fn() -> impl Responder)
            .service(get_projects)
            .service(get_project)
            .service(get_image)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
