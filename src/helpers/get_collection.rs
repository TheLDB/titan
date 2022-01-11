pub struct Collection {
    floor_price: json::JsonValue,
}

impl Collection {
    pub async fn get(collection_slug: &str) -> Result<(), Box<dyn std::error::Error>>  {
        let resp = reqwest::get(format!("https://api.opensea.io/api/v1/collection/{}", collection_slug)).await?;
        println!("Status: {}", resp.status());
        
        let body = resp.text().await?;
        let parsed = json::parse(&body).unwrap();
        let stats = &parsed["collection"]["stats"];
        println!("Body: \n{} ETH", stats["floor_price"]);
        Ok(())
    }
}
