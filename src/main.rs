mod commands;
mod services;
mod utils;

use dotenv::dotenv;
use commands::general::GENERAL_GROUP;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} est connecté!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let token = std::env::var("DISCORD_TOKEN").expect("Token non trouvé");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Erreur lors de la création du client");

    if let Err(why) = client.start().await {
        println!("Une erreur s'est produite lors de l'exécution du client: {:?}", why);
    }
}