use serde_json::{Value};

#[derive(Debug)]
pub struct Collection {
    pub status: reqwest::StatusCode,
    pub name: Value,
    pub fp: Value,
    pub stats: Value,
}

impl Collection {
    pub async fn get(collection_slug: &str) -> Result<Collection, Box<dyn std::error::Error>>   {
        let resp = reqwest::get(format!("https://api.opensea.io/api/v1/collection/{}", collection_slug)).await?;
        let status = resp.status();
        
        let body = resp.text().await?;
        let v: Value = serde_json::from_str(&body)?;
        let new = Collection {
            status: status,
            name: v["collection"]["name"].clone(),
            fp: v["collection"]["stats"]["floor_price"].clone(),
            stats: v["collection"]["stats"].clone(),
        };
        Ok(new)
    }
}
