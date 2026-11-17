fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    lumen::run()
}
