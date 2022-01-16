type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Monitor a new collection
#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn monitor(
    ctx: Context<'_>,
    #[description = "Collection Slug"] slug: Option<String>,
    #[description = "Amount the floor price has to move up or down"] threshold: Option<i32>,
    #[description = "Send a notification if volume spikes?"] volume: Option<bool>,
) -> Result<(), Error> {
    // Eventually, this will connect to a database, and there will be tables such as:
    // Guilds (holds all guilds)
    // Collections (holds all collections being monitored, and assigns them to a guild, repeated slugs are allowed)
    
    // This will be paired with a worker service that gets the floor of every collection every x minutes and alerts if volume is above regular
    ctx.say("Eventually, this will connect to a database, and there will be tables that hold all guilds, and all collections to be monitored.\nIn the future, this will be paired with a worker service that gets the floor once every x minutes, and tracks all activity, and will alert if the activity/volume spikes").await?;

    Ok(())
}