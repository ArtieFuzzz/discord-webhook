use discord_webhook::DiscordWebhook;
use std::env::var;

#[tokio::test]
async fn text_test() {
  let url = var("DISCORD_HOOK").unwrap();
  let hook = DiscordWebhook::new(url);

  hook.send_text("Hello World!").await.unwrap();
}