use discord_webhook::{DiscordWebhook, structures::WebhookMessage};
use std::env::var;

#[tokio::test]
async fn text_test() {
    let url = var("DISCORD_HOOK").unwrap();
    let hook = DiscordWebhook::new(url);

    hook.send_text("Hello World!").await.unwrap();
}

#[tokio::test]
async fn raw_test() {
    let url = var("DISCORD_HOOK").unwrap();
    let hook = DiscordWebhook::new(url);

    let message = WebhookMessage {
      content: Some("Hello World!".to_owned()),
      username: Some("This changed".to_owned()),
      embed: None,
      avatar_url: None,
      allowed_mentions: None,
    };

    hook.send_raw(&message).await.unwrap();
}
