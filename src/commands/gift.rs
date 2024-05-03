use poise::serenity_prelude as serenity;

use crate::database;
use crate::Context;
use crate::Error;

#[poise::command(slash_command, prefix_command)]
pub async fn gift(
    ctx: Context<'_>,
    #[description = "Selected user"] user: serenity::User,
    #[description = "Amount"] amount: f64
) -> Result<(), Error> {
    let data = &ctx.data().user_data;
    match database::users::gift(&data.data, &ctx.author().id.to_string(), &user.id.to_string(), amount) {
        Some(_) => {
            ctx.say(format!("Gifted {} spirit dollars to {}", amount, user.name)).await?;
        },
        None => {
            ctx.say(format!("Nice try but you're broke lmao")).await?;
        }
    }
    database::sync(data).unwrap();
    Ok(())
}
