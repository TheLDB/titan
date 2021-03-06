use serde_json::{Value};

#[derive(Debug)]
pub struct Collection {
    pub status: reqwest::StatusCode,
    pub name: Value,
    pub fp: Value,
    pub stats: Value,
    pub img: Value,
    pub supply: Value,
    pub num_owners: Value,
    pub day_vol: Value,
}

impl Collection {
    pub async fn get(collection_slug: &str) -> Result<Collection, Box<dyn std::error::Error + Send + Sync>> {
        let resp = reqwest::get(format!("https://api.opensea.io/api/v1/collection/{}", collection_slug)).await?;
        let status = resp.status();
        
        let body = resp.text().await?;
        let v: Value = serde_json::from_str(&body)?;
        let new = Collection {
            status: status,
            name: v["collection"]["name"].clone(),
            fp: v["collection"]["stats"]["floor_price"].clone(),
            stats: v["collection"]["stats"].clone(),
            img: v["collection"]["image_url"].clone(),
            supply: v["collection"]["stats"]["total_supply"].clone(),
            num_owners: v["collection"]["stats"]["num_owners"].clone(),
            day_vol: v["collection"]["stats"]["one_day_volume"].clone(),
        };
        Ok(new)
    }
}
