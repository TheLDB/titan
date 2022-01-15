type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Get collection stats from OpenSea
#[poise::command(slash_command, track_edits)]
pub async fn collection(
    ctx: Context<'_>,
    #[description = "Collection Slug"] slug: String,
) -> Result<(), Error> {
    println!("{}", slug);
    ctx.say(format!("{}", slug)).await?;
    Ok(())
}