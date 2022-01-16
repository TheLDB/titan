type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use poise::serenity_prelude as serenity;

/// Show all commands
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.send(|m| {
        m.embed(|e| {
            e.title("Titan Commands");
            e.description("Full command list for the OpenSea Monitor & Info Bot, Titan.");
            e.color(serenity::Colour::BLURPLE);
            e.field("/collection ``collection-slug``", "Get the details of a collection by using their slug. Use /slug if you want more info on how to find a collection slug.", false);
            e.field("/slug", "Get help on how to find a collection slug", false);
            e.field("/source", "View the source code of Titan on github", false);
            e.field("/donate", "Donate to keep Titan running", false)
        })
    }).await?;

    Ok(())
}