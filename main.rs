// Simple Discord bot (Rust). Single file, no database.
// Run: DISCORD_TOKEN=xxx cargo run --release
//
// Components V2 is sent as raw JSON over the REST API, so it does not depend
// on serenity having builders for it.

use serde_json::json;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

const REACT_WORD: &str = "pizza"; // reacts with a pizza emoji
const REACT_EMOJI: char = '🍕';
const EMBED_WORD: &str = "embed"; // replies with a Components V2 container

struct Handler {
    http: reqwest::Client,
    token: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let text = msg.content.to_lowercase();

        // Basic commands
        match text.trim() {
            "!ping" => {
                let _ = msg.reply(&ctx.http, "Pong!").await;
                return;
            }
            "!hello" => {
                let _ = msg
                    .reply(&ctx.http, format!("Hello, {}!", msg.author.name))
                    .await;
                return;
            }
            "!help" => {
                let _ = msg
                    .reply(
                        &ctx.http,
                        format!(
                            "Commands: `!ping`, `!hello`, `!help`\n\
                             Say \"{REACT_WORD}\" and I react. Say \"{EMBED_WORD}\" and I send a Components V2 message."
                        ),
                    )
                    .await;
                return;
            }
            _ => {}
        }

        // React if the word is in the message
        if text.contains(REACT_WORD) {
            if let Err(e) = msg.react(&ctx.http, REACT_EMOJI).await {
                eprintln!("react failed: {e}");
            }
        }

        // Components V2 message if the word is in the message
        if text.contains(EMBED_WORD) {
            let body = json!({
                "flags": 1 << 15, // IS_COMPONENTS_V2
                "message_reference": { "message_id": msg.id.get().to_string() },
                "components": [{
                    "type": 17, // container
                    "accent_color": 0x5865F2,
                    "components": [
                        { "type": 10, "content": "## Components V2" },
                        { "type": 14, "divider": true, "spacing": 1 },
                        { "type": 10, "content": format!("You said the magic word, **{}**.", msg.author.name) }
                    ]
                }]
            });

            let url = format!("https://discord.com/api/v10/channels/{}/messages", msg.channel_id);
            match self
                .http
                .post(url)
                .header("Authorization", format!("Bot {}", self.token))
                .json(&body)
                .send()
                .await
            {
                Ok(r) if !r.status().is_success() => {
                    eprintln!("components v2 send failed: {}", r.status());
                }
                Err(e) => eprintln!("components v2 send failed: {e}"),
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), serenity::Error> {
    let token = std::env::var("DISCORD_TOKEN").expect("set DISCORD_TOKEN");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT; // enable in the developer portal too

    let handler = Handler {
        http: reqwest::Client::new(),
        token: token.clone(),
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

    client.start().await
}
