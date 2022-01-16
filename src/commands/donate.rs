type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Support the only dev/maintainer
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn donate(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Thanks for using this bot! My name is Landon, and I am the sole developer/maintainer of this bot. If you'd like to support my work, I accept donations in both ETH and SOL!\nETH Address: 0xEf2abE8d4307Fc8AFaa6c13AcA7a359A706D6BeE\nSOL Address: Ak1vYFLAUrvLESj6NpnEQqfCUMDQbFjkYiNGcFyf2aBc\nIf you'd just like to drop me a follow on twitter, that's cool too!\nhttps://twitter.com/theldb_").await?;

    Ok(())
}