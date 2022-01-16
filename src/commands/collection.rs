type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use crate::helpers;
use helpers::get_collection::Collection;
use poise::serenity_prelude as serenity;
// Get collection stats from OpenSea
#[poise::command(slash_command, track_edits)]
pub async fn collection(
    ctx: Context<'_>,
    #[description = "Collection Slug"] slug: String,
) -> Result<(), Error> {
    if let Ok(res) = Collection::get(&slug).await {
        let floor = if res.fp.to_string() == "null" {
            "None Listed!".to_string()
        } else { format!("{} Ξ", res.fp.to_string()) };

        let vol = if res.day_vol.to_string() == "0.0" {
            "No Volume :(".to_string()
        } else { format!("{:.6} Ξ", res.day_vol.to_string()) };

        ctx.send(|m| {
            m.embed(|e| {
                e.title(format!("{} Statistics", res.name.to_string().replace('"', "")));
                e.description(format!("Statistics for {} provided by OpenSea", res.name.to_string().replace('"', "")));
                e.field("Floor Price", format!("{}", floor), true);
                e.field("Total Supply", format!("{}", res.supply.to_string().split(".").nth(0).unwrap()), true);
                e.field("Owners", format!("{}", res.num_owners.to_string()), true);
                e.field("24 Hr Volume", format!{"{}", vol}, true);
                e.image(format!("{}", res.img.to_string().replace('"', "")));
                e.colour(serenity::Color::BLURPLE)
            })
        }).await?;
    }
    else {

    }
    Ok(())
}