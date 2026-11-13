use std::env;
use tapo::{ApiClient, requests::Color};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;

    println!("{}", ip_address);
    let device = ApiClient::new(tapo_username, tapo_password)
        .l630(ip_address)
        .await?;

    device.on().await?;

    device
        .set()
        .brightness(30)
        .color(Color::Aquamarine)
        .send(&device)
        .await?;

    Ok(())
}
