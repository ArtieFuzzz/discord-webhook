use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookMessage {
    pub content: Option<String>,
    pub username: Option<String>,
    pub embed: Option<Vec<DiscordEmbed>>,
    pub avatar_url: Option<String>,
    pub allowed_mentions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<i64>,
    pub color: Option<i16>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedImage>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedImage {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedAuthor {
  pub name: String,
  pub url: Option<String>,
  pub icon_url: Option<String>,
}