type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Get the bot's source cdoe
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn source(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("This bot was built in rust, and is fully open source. If you'd like to take a look, it's on github! https://github.com/TheLDB/titan\nIf you'd like to support me, use ``/donate`` or ``/support``").await?;

    Ok(())
}