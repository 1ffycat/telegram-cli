use anyhow::{Context, Result};
use reqwest::{Proxy, Response, multipart};
use serde::Deserialize;
use std::path::Path;
use std::time::Duration;

/// How outgoing requests reach the network.
pub enum ProxyMode {
    /// Honour HTTPS_PROXY / HTTP_PROXY / ALL_PROXY / NO_PROXY.
    System,
    /// Always use this proxy.
    Url(String),
    /// Never use a proxy.
    Disabled,
}

#[derive(Clone, Copy)]
pub enum Media {
    Document,
    Photo,
    Video,
    Audio,
}

impl Media {
    fn method(self) -> &'static str {
        match self {
            Media::Document => "sendDocument",
            Media::Photo => "sendPhoto",
            Media::Video => "sendVideo",
            Media::Audio => "sendAudio",
        }
    }

    fn field(self) -> &'static str {
        match self {
            Media::Document => "document",
            Media::Photo => "photo",
            Media::Video => "video",
            Media::Audio => "audio",
        }
    }
}

pub struct TelegramClient {
    client: reqwest::Client,
    bot_token: String,
}

impl TelegramClient {
    pub fn new(bot_token: String, proxy: ProxyMode) -> Result<Self> {
        let builder = reqwest::Client::builder().connect_timeout(Duration::from_secs(15));
        let builder = match proxy {
            ProxyMode::System => builder,
            ProxyMode::Disabled => builder.no_proxy(),
            // The URL may carry credentials, so it is deliberately left out of the error.
            ProxyMode::Url(url) => builder.proxy(
                Proxy::all(url)
                    .map_err(|e| e.without_url())
                    .context("Invalid proxy URL")?,
            ),
        };
        Ok(Self {
            client: builder.build().context("Failed to build HTTP client")?,
            bot_token,
        })
    }

    fn url(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.bot_token, method)
    }

    pub async fn send_message(
        &self,
        chat_id: &str,
        text: &str,
        parse_mode: Option<&str>,
    ) -> Result<()> {
        let mut params = vec![("chat_id", chat_id), ("text", text)];
        if let Some(pm) = parse_mode {
            params.push(("parse_mode", pm));
        }

        let response = self
            .client
            .post(self.url("sendMessage"))
            .form(&params)
            .send()
            .await
            .map_err(|e| e.without_url())
            .context("Failed to reach Telegram")?;
        check(response, "Failed to send message").await
    }

    pub async fn send_media(
        &self,
        media: Media,
        chat_id: &str,
        file_path: &Path,
        caption: Option<&str>,
        parse_mode: Option<&str>,
    ) -> Result<()> {
        // Streamed from disk, so large files are not held in memory.
        let part = multipart::Part::file(file_path)
            .await
            .with_context(|| format!("Failed to read file: {file_path:?}"))?;

        let mut form = multipart::Form::new()
            .text("chat_id", chat_id.to_string())
            .part(media.field(), part);

        if let Some(cap) = caption {
            form = form.text("caption", cap.to_string());
            if let Some(pm) = parse_mode {
                form = form.text("parse_mode", pm.to_string());
            }
        }

        let response = self
            .client
            .post(self.url(media.method()))
            .multipart(form)
            .send()
            .await
            .map_err(|e| e.without_url())
            .context("Failed to reach Telegram")?;
        check(
            response,
            &format!("Failed to send {} {file_path:?}", media.field()),
        )
        .await
    }
}

#[derive(Deserialize)]
struct ApiError {
    description: Option<String>,
}

/// Turns a non-2xx response into an error carrying Telegram's own explanation.
/// (`error_for_status` would put the request URL, and thus the bot token, in the message.)
async fn check(response: Response, what: &str) -> Result<()> {
    let status = response.status();
    if status.is_success() {
        return Ok(());
    }
    let description = response
        .text()
        .await
        .ok()
        .and_then(|body| serde_json::from_str::<ApiError>(&body).ok())
        .and_then(|e| e.description);
    match description {
        Some(d) => anyhow::bail!("{what}: {status}: {d}"),
        None => anyhow::bail!("{what}: {status}"),
    }
}
