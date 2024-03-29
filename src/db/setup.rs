use anyhow::Result;
use tokio_postgres::Client;

pub async fn setup() -> Result<()> {
    let client: Client = todo!();
    Ok(())
}
