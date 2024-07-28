mod server;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        server::start_server().await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    client::run_client().await?;

    Ok(())
}
