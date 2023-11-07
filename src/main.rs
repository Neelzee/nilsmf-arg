mod utils;
mod api;

use actix_web::{App, HttpServer};
use api::articles::get_article;
use api::images::get_image;
use api::projects::{get_project, get_projects};
use actix_cors::Cors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            // fn service(f: impl Fn() -> impl Responder)
            .service(get_projects)
            .service(get_article)
            .service(get_project)
            .service(get_image)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
