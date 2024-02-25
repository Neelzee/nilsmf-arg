mod api;
// mod db;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use api::articles::get_article;
use api::images::get_image;
use api::projects::{get_project, get_projects};
use openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslMethod},
};
use std::{
    fs::File,
    io::{self, Read as _},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting backend");
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    // set the encrypted private key
    builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();

    builder
        .set_certificate_chain_file("cert.pem")
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            // fn service(f: impl Fn() -> impl Responder)
            .service(get_projects)
            .service(get_article)
            .service(get_project)
            .service(get_image)
    })
    .bind_openssl(("0.0.0.0", 8080), builder)?
    // .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn load_encrypted_private_key() -> PKey<Private> {
    let mut file = File::open("privkey.pem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    PKey::private_key_from_pem_passphrase(&buffer, b"password").unwrap()
}
