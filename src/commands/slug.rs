type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use poise::serenity_prelude as serenity;

/// Show all commands
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn slug(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.send(|m| {
        m.embed(|e| {
            e.title("How to find a collection slug");
            e.color(serenity::Colour::BLURPLE);
            e.field("What is a slug?", "A collection slug is the hyphenated collection name that OpenSea uses.\nFor example, the link for Doodles is ``https://opensea.io/collection/doodles-official``\nThe slug of this would be ``doodles-official``.", false)
        })
    }).await?;

    Ok(())
}