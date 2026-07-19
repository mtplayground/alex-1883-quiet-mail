#![allow(dead_code)]

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    config::{Config, EmailConfig},
    db::Database,
    error::AppError,
    mail::{insert_sent_message, snippet_from_body, Message, SentMessageInsert},
};

#[derive(Clone)]
pub struct OutboundEmailService {
    adapter: Option<PlatformEmailAdapter>,
}

#[derive(Clone)]
struct PlatformEmailAdapter {
    client: reqwest::Client,
    url: String,
    app_token: String,
}

#[derive(Debug, Clone)]
pub struct OutboundEmail {
    pub sender: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub reply_to: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SentMailRecord {
    pub message: Message,
    pub delivery: EmailDelivery,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailDelivery {
    Sent { provider_message_id: String },
    Skipped { reason: String },
}

#[derive(Debug, Serialize)]
struct PlatformEmailRequest<'a> {
    to: &'a [String],
    subject: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct PlatformEmailResponse {
    id: String,
}

impl OutboundEmailService {
    pub fn from_config(config: &Config) -> Self {
        Self {
            adapter: config.email.as_ref().map(PlatformEmailAdapter::from_config),
        }
    }

    pub async fn send_and_record(
        &self,
        database: &Database,
        email: OutboundEmail,
    ) -> Result<SentMailRecord, AppError> {
        email.validate()?;

        let delivery = match &self.adapter {
            Some(adapter) => adapter.send(&email).await?,
            None => EmailDelivery::Skipped {
                reason: "email service is not configured".to_owned(),
            },
        };

        let message = insert_sent_message(database, email.into_sent_message_insert()).await?;

        Ok(SentMailRecord { message, delivery })
    }
}

impl PlatformEmailAdapter {
    fn from_config(config: &EmailConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: config.url.clone(),
            app_token: config.app_token.clone(),
        }
    }

    async fn send(&self, email: &OutboundEmail) -> Result<EmailDelivery, AppError> {
        let request = PlatformEmailRequest {
            to: &email.to,
            subject: &email.subject,
            html: email.html.as_deref(),
            text: email.text.as_deref(),
            reply_to: email.reply_to.as_deref(),
        };

        let response = self
            .client
            .post(&self.url)
            .bearer_auth(&self.app_token)
            .json(&request)
            .send()
            .await
            .map_err(|source| AppError::Email {
                detail: source.to_string(),
            })?;

        let status = response.status();
        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(AppError::Email {
                detail: "email service rate limited the request".to_owned(),
            });
        }

        if !status.is_success() {
            let body = match response.text().await {
                Ok(body) => body,
                Err(error) => format!("failed to read response body: {error}"),
            };
            return Err(AppError::Email {
                detail: format!("email service returned {status}: {body}"),
            });
        }

        response
            .json::<PlatformEmailResponse>()
            .await
            .map(|body| EmailDelivery::Sent {
                provider_message_id: body.id,
            })
            .map_err(|source| AppError::Email {
                detail: source.to_string(),
            })
    }
}

impl OutboundEmail {
    fn validate(&self) -> Result<(), AppError> {
        if self.to.is_empty() {
            return Err(AppError::Email {
                detail: "outbound email requires at least one recipient".to_owned(),
            });
        }

        if self.subject.trim().is_empty() {
            return Err(AppError::Email {
                detail: "outbound email requires a subject".to_owned(),
            });
        }

        if self
            .html
            .as_ref()
            .map_or(true, |value| value.trim().is_empty())
            && self
                .text
                .as_ref()
                .map_or(true, |value| value.trim().is_empty())
        {
            return Err(AppError::Email {
                detail: "outbound email requires html or text content".to_owned(),
            });
        }

        Ok(())
    }

    fn into_sent_message_insert(self) -> SentMessageInsert {
        let body = self
            .html
            .as_ref()
            .filter(|value| !value.trim().is_empty())
            .or(self.text.as_ref())
            .cloned()
            .unwrap_or_default();
        let snippet = snippet_from_body(&body);

        SentMessageInsert {
            sender: self.sender,
            to_recipients: self.to,
            cc_recipients: self.cc,
            bcc_recipients: self.bcc,
            subject: self.subject,
            body,
            snippet,
        }
    }
}
