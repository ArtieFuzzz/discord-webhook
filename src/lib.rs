pub mod structures;

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Serialize;
use serde_json::to_string_pretty;

static HTTP: Lazy<Client> = Lazy::new(Client::new);

pub struct DiscordWebhook {
  hook_url: Option<String>
}

fn stringify_struct<T: Serialize>(from: &T) -> Result<String> {
  return Ok(to_string_pretty(from)?)
}

impl DiscordWebhook {
  pub fn new(hook_url: String) -> Self {
    DiscordWebhook {
      hook_url: Some(hook_url)
    }
  }

  pub async fn send_text(&self, message: &str) -> Result<()> {
    if let Some(url) = &self.hook_url {
      let wmessage = structures::WebhookMessage {
        content: Some(message.to_owned()),
        username: None,
        embed: None,
        avatar_url: None,
        allowed_mentions: None,
    };

      let body = stringify_struct(&wmessage)?;

      HTTP.post(url).header("Content-Type", "application/json").body(body).send().await?;
      return Ok(())
    }

    Err(anyhow!("You must set the hook url via .set_url"))
  }
}
