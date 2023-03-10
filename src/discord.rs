use std::{env, error::Error, sync::Arc};
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::{request::channel::reaction::RequestReactionType, Client as HttpClient};

use super::rate;

pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("TOKEN")?;

    let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
    let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);
    let http = Arc::new(HttpClient::new(token.clone()));

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(why) => {
                println!("Error: {why:?}");
                continue;
            }
        };

        tokio::spawn(handle(event, Arc::clone(&http)));
    }
}

async fn handle(event: Event, http: Arc<HttpClient>) {
    #[allow(clippy::single_match)]
    match event {
        Event::MessageCreate(message) => {
            let (score, _) = rate::rate(message.content.to_string());

            if score == 12 {
                let result = http
                    .create_reaction(
                        message.channel_id,
                        message.id,
                        &RequestReactionType::Unicode { name: "⭐" },
                    )
                    .await;

                if let Err(why) = result {
                    println!("Error: {why:?}");
                }
            }
        }
        _ => {}
    }
}
