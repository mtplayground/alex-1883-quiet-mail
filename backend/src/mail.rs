#![allow(dead_code)]

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, db::Database, error::AppError};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, sqlx::FromRow)]
pub struct FolderSummary {
    pub key: String,
    pub display_name: String,
    pub sort_order: i16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, sqlx::FromRow)]
pub struct MessageListItem {
    pub id: i64,
    pub sender: String,
    pub subject: String,
    pub snippet: String,
    pub sent_at: DateTime<Utc>,
    pub is_read: bool,
}

#[derive(Debug, Serialize)]
pub struct FoldersResponse {
    pub folders: Vec<FolderSummary>,
}

#[derive(Debug, Serialize)]
pub struct MessagesResponse {
    pub folder_key: String,
    pub messages: Vec<MessageListItem>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/folders", get(list_folders))
        .route("/folders/:folder_key/messages", get(list_messages))
}

async fn list_folders(State(state): State<AppState>) -> Result<Json<FoldersResponse>, AppError> {
    let folders = fetch_folders(&state.database).await?;

    Ok(Json(FoldersResponse { folders }))
}

async fn list_messages(
    State(state): State<AppState>,
    Path(folder_key): Path<String>,
) -> Result<Json<MessagesResponse>, AppError> {
    let folder = SystemFolder::from_key(&folder_key).ok_or_else(|| AppError::NotFound {
        message: "folder not found".to_owned(),
    })?;
    let messages = fetch_messages_for_folder(&state.database, folder).await?;

    Ok(Json(MessagesResponse {
        folder_key: folder.key().to_owned(),
        messages,
    }))
}

pub async fn fetch_folders(database: &Database) -> Result<Vec<FolderSummary>, AppError> {
    sqlx::query_as::<_, FolderSummary>(
        r#"
        SELECT key, display_name, sort_order
        FROM folders
        ORDER BY sort_order ASC
        "#,
    )
    .fetch_all(database.pool())
    .await
    .map_err(|source| AppError::Database { source })
}

pub async fn fetch_messages_for_folder(
    database: &Database,
    folder: SystemFolder,
) -> Result<Vec<MessageListItem>, AppError> {
    sqlx::query_as::<_, MessageListItem>(
        r#"
        SELECT id, sender, subject, snippet, sent_at, is_read
        FROM messages
        WHERE folder_key = $1
        ORDER BY sent_at DESC, id DESC
        "#,
    )
    .bind(folder.key())
    .fetch_all(database.pool())
    .await
    .map_err(|source| AppError::Database { source })
}
