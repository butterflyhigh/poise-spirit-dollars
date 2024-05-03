use std::{fs, path};
use dotenv;
use poise::serenity_prelude as serenity;
use tokio;

mod commands;
mod database;

use database::{sync, users::UserData, Database};

struct Data {
    user_data: Database<UserData>
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const DATA_SUBDIR: &str = "./data";
const USER_DB_FILE: &str = "userdb.json";

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let binding = path::Path::new(DATA_SUBDIR)
        .join(USER_DB_FILE);
    let path = binding
        .as_os_str()
        .to_str()
        .unwrap();

    let db: Database<UserData> = match fs::read(path) {
        Ok(_) => {
            println!("Opening db");
            database::open_database(path).unwrap()
        }

        Err(_) => {
            println!("Creating db");
            database::create_database(path).unwrap()
        }
    };

    sync(&db).unwrap();

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::inflate_market::inflate_market(),
                commands::get_balance::get_balance(),
                commands::gift::gift()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { user_data: db })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}
