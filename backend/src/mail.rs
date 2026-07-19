#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SystemFolder {
    Inbox,
    Sent,
    Drafts,
    Archive,
    Trash,
}

impl SystemFolder {
    pub const ALL: [Self; 5] = [
        Self::Inbox,
        Self::Sent,
        Self::Drafts,
        Self::Archive,
        Self::Trash,
    ];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Inbox => "inbox",
            Self::Sent => "sent",
            Self::Drafts => "drafts",
            Self::Archive => "archive",
            Self::Trash => "trash",
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Inbox => "Inbox",
            Self::Sent => "Sent",
            Self::Drafts => "Drafts",
            Self::Archive => "Archive",
            Self::Trash => "Trash",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "inbox" => Some(Self::Inbox),
            "sent" => Some(Self::Sent),
            "drafts" => Some(Self::Drafts),
            "archive" => Some(Self::Archive),
            "trash" => Some(Self::Trash),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Folder {
    pub key: String,
    pub display_name: String,
    pub sort_order: i16,
    pub system: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: i64,
    pub folder_key: String,
    pub sender: String,
    pub to_recipients: Vec<String>,
    pub cc_recipients: Vec<String>,
    pub bcc_recipients: Vec<String>,
    pub subject: String,
    pub body: String,
    pub snippet: String,
    pub sent_at: DateTime<Utc>,
    pub is_read: bool,
    pub thread_root_id: Option<i64>,
    pub reply_to_message_id: Option<i64>,
    pub forwarded_from_message_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
