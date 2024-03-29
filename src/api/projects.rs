use std::fs::File;
use std::io::{self, Read};
use crate::utils::consts::PATH;

use crate::utils::funcs::get_file_content;

//#[get("/projects/all")]
/*
pub async fn get_projects() -> impl Responder {
    let res: Result<Vec<(String, String)>, io::Error> = async {
        let mut file_contents: Vec<(String, String)> = Vec::new();

        for entry in WalkDir::new(format!("{}{}", PATH, "markdown/recap-project/")).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path();

                let mut file = File::open(file_path)?;
                let mut file_content = String::new();
                file.read_to_string(&mut file_content)?;

                if let Some(file_name) = entry.file_name().to_str() {
                    file_contents.push((file_name.to_string(), file_content));
                }

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


#[get("/project/{file}")]
pub async fn get_project(path: web::Path<String>) -> impl Responder {
    let file_path = path.into_inner() + ".md";

    let res = get_file_content(format!("{}{}{}", PATH, "markdown/projects/", &file_path));
    
    if let Ok(con) = res {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Content-Type", "text/markdown"))
            .body(con);
    }

    HttpResponse::NotFound().insert_header(("Access-Control-Allow-Origin", "*")).body(String::from("File not found: ".to_string() + &file_path))
}
*/
