pub mod helpers;
use helpers::get_collection::Collection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Collection::get("doodles-official").await?;
    Ok(())
}