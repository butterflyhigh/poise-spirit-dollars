use poise::serenity_prelude as serenity;

use crate::database;
use crate::Context;
use crate::Error;

#[poise::command(slash_command, prefix_command)]
pub async fn inflate_market(
    ctx: Context<'_>,
    #[description = "Selected user"] user: serenity::User,
    #[description = "Amount"] amount: f64
) -> Result<(), Error> {
    let data = &ctx.data().user_data;
    database::users::add_balance(&data.data, &user.id.to_string(), amount);
    database::sync(data).unwrap();
    ctx.say(format!("Destroyed the economy by adding {} spirit dollars to {} account", amount, user)).await?;
    Ok(())
}
