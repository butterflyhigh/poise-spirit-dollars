use poise::serenity_prelude as serenity;

use crate::database;
use crate::Context;
use crate::Error;

#[poise::command(slash_command, prefix_command)]
pub async fn get_balance(
    ctx: Context<'_>,
    #[description = "Selected user"] user: serenity::User,
) -> Result<(), Error> {
    let data = &ctx.data().user_data;
    let amt = database::users::get(&data.data, &user.id.to_string());
    ctx.say(format!("{} has {} spirit dollars", user.tag(), amt.unwrap_or_default())).await?;
    Ok(())
}
