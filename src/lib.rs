pub mod structures;

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Serialize;
use serde_json::to_string_pretty;
use structures::WebhookMessage;

static HTTP: Lazy<Client> = Lazy::new(Client::new);

pub struct DiscordWebhook {
    hook_url: Option<String>,
}

fn stringify_struct<T: Serialize>(from: &T) -> Result<String> {
    return Ok(to_string_pretty(from)?);
}

impl DiscordWebhook {
    pub fn new(hook_url: String) -> Self {
        DiscordWebhook {
            hook_url: Some(hook_url),
        }
    }

    /// Send text to the Discord Webhook
    ///
    /// ```rs
    /// use discord_webhook::DiscordWebhook;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let url = var("DISCORD_HOOK").unwrap();
    ///   let hook = DiscordWebhook::new(url);
    ///   hook.send_text("Hello World!").await.unwrap();
    /// }
    /// ```
    pub async fn send_text(&self, message: &str) -> Result<()> {
        // Make sending messages somewhat shorter
        if let Some(url) = &self.hook_url {
            let wmessage = structures::WebhookMessage {
                content: Some(message.to_owned()),
                username: None,
                embed: None,
                avatar_url: None,
                allowed_mentions: None,
            };

            let body = stringify_struct(&wmessage)?;

            HTTP.post(url)
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await?;
            return Ok(());
        }

        Err(anyhow!("You must set the hook url via ::new"))
    }

    /// Send a custom message to the Discord Webhook
    ///
    /// ```rs
    /// use discord_webhook::DiscordWebhook;
    ///
    /// let message = WebhookMessage {
    ///   content: Some("Hello World!".to_owned()),
    ///   username: Some("This changed".to_owned()),
    ///   embed: None,
    ///   avatar_url: None,
    ///   allowed_mentions: None,
    ///   avatar_url
    /// };
    /// 
    /// hook.send_raw(&message).await.unwrap();
    /// ```
    pub async fn send_raw(&self, data: &WebhookMessage) -> Result<()> {
        if let Some(url) = &self.hook_url {
            let body = stringify_struct(&data)?;

            HTTP.post(url)
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await?;
            return Ok(());
        }

        Err(anyhow!("You must set the hook url via ::new"))
    }
}
