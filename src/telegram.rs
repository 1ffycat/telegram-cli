use anyhow::{Context, Result};
use reqwest::multipart;
use std::path::Path;

pub struct TelegramClient {
    client: reqwest::Client,
    bot_token: String,
}

impl TelegramClient {
    pub fn new(bot_token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            bot_token,
        }
    }

    pub async fn send_message(
        &self,
        chat_id: &str,
        text: &str,
        parse_mode: Option<&str>,
    ) -> Result<()> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);

        let mut params = std::collections::HashMap::new();
        params.insert("chat_id", chat_id);
        params.insert("text", text);

        if let Some(pm) = parse_mode {
            params.insert("parse_mode", pm);
        }

        self.client
            .post(&url)
            .form(&params)
            .send()
            .await?
            .error_for_status()
            .context("Failed to send message")?;
        Ok(())
    }

    pub async fn send_document(
        &self,
        chat_id: &str,
        file_path: &Path,
        caption: Option<&str>,
        parse_mode: Option<&str>,
    ) -> Result<()> {
        self.send_file(
            "sendDocument",
            "document",
            chat_id,
            file_path,
            caption,
            parse_mode,
        )
        .await
    }

    pub async fn send_photo(
        &self,
        chat_id: &str,
        file_path: &Path,
        caption: Option<&str>,
        parse_mode: Option<&str>,
    ) -> Result<()> {
        self.send_file(
            "sendPhoto",
            "photo",
            chat_id,
            file_path,
            caption,
            parse_mode,
        )
        .await
    }

    pub async fn send_video(
        &self,
        chat_id: &str,
        file_path: &Path,
        caption: Option<&str>,
        parse_mode: Option<&str>,
    ) -> Result<()> {
        self.send_file(
            "sendVideo",
            "video",
            chat_id,
            file_path,
            caption,
            parse_mode,
        )
        .await
    }

    pub async fn send_audio(
        &self,
        chat_id: &str,
        file_path: &Path,
        caption: Option<&str>,
        parse_mode: Option<&str>,
    ) -> Result<()> {
        self.send_file(
            "sendAudio",
            "audio",
            chat_id,
            file_path,
            caption,
            parse_mode,
        )
        .await
    }

    async fn send_file(
        &self,
        method: &str,
        file_type: &str,
        chat_id: &str,
        file_path: &Path,
        caption: Option<&str>,
        parse_mode: Option<&str>,
    ) -> Result<()> {
        let url = format!("https://api.telegram.org/bot{}/{}", self.bot_token, method);
        let file_name = file_path
            .file_name()
            .context("Invalid file path")?
            .to_string_lossy()
            .to_string();
        let file_bytes = tokio::fs::read(file_path)
            .await
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;
        let part = multipart::Part::bytes(file_bytes).file_name(file_name);

        let mut form = multipart::Form::new()
            .text("chat_id", chat_id.to_string())
            .part(file_type.to_string(), part);

        if let Some(cap) = caption {
            form = form.text("caption", cap.to_string());
            if let Some(pm) = parse_mode {
                form = form.text("parse_mode", pm.to_string());
            }
        }

        self.client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .error_for_status()
            .with_context(|| format!("Failed to send {} from path {:?}", file_type, file_path))?;
        Ok(())
    }
}
