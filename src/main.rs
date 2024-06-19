mod api;
mod db;
mod utils;

use rocket::{self, routes};
use rocket_errors::anyhow::Result;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[rocket::main]
async fn main() -> Result<()> {
    let _rocket = rocket::build()
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .launch()
        .await?;
    Ok(())
}
