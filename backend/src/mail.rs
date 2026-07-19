#![allow(dead_code)]

use axum::{
    extract::{Path, State},
    routing::{get, post},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MoveAction {
    Archive,
    Trash,
    Restore,
}

impl MoveAction {
    pub const fn target_folder(self) -> SystemFolder {
        match self {
            Self::Archive => SystemFolder::Archive,
            Self::Trash => SystemFolder::Trash,
            Self::Restore => SystemFolder::Inbox,
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

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct MoveMessageRequest {
    pub action: MoveAction,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/folders", get(list_folders))
        .route("/folders/:folder_key/messages", get(list_messages))
        .route("/messages/:message_id", get(message_detail))
        .route("/messages/:message_id/move", post(move_message))
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

async fn message_detail(
    State(state): State<AppState>,
    Path(message_id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let message = fetch_message_detail_and_mark_read(&state.database, message_id).await?;

    Ok(Json(MessageResponse { message }))
}

async fn move_message(
    State(state): State<AppState>,
    Path(message_id): Path<i64>,
    Json(request): Json<MoveMessageRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let message =
        move_message_to_folder(&state.database, message_id, request.action.target_folder()).await?;

    Ok(Json(MessageResponse { message }))
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

pub async fn fetch_message_detail_and_mark_read(
    database: &Database,
    message_id: i64,
) -> Result<Message, AppError> {
    let mut transaction = database
        .pool()
        .begin()
        .await
        .map_err(|source| AppError::Database { source })?;

    sqlx::query(
        r#"
        UPDATE messages
        SET is_read = TRUE
        WHERE id = $1 AND is_read = FALSE
        "#,
    )
    .bind(message_id)
    .execute(&mut *transaction)
    .await
    .map_err(|source| AppError::Database { source })?;

    let message = sqlx::query_as::<_, Message>(
        r#"
        SELECT
            id,
            folder_key,
            sender,
            to_recipients,
            cc_recipients,
            bcc_recipients,
            subject,
            body,
            snippet,
            sent_at,
            is_read,
            thread_root_id,
            reply_to_message_id,
            forwarded_from_message_id,
            created_at,
            updated_at
        FROM messages
        WHERE id = $1
        "#,
    )
    .bind(message_id)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|source| AppError::Database { source })?
    .ok_or_else(|| AppError::NotFound {
        message: "message not found".to_owned(),
    })?;

    transaction
        .commit()
        .await
        .map_err(|source| AppError::Database { source })?;

    Ok(message)
}

pub async fn move_message_to_folder(
    database: &Database,
    message_id: i64,
    folder: SystemFolder,
) -> Result<Message, AppError> {
    sqlx::query_as::<_, Message>(
        r#"
        UPDATE messages
        SET folder_key = $2
        WHERE id = $1
        RETURNING
            id,
            folder_key,
            sender,
            to_recipients,
            cc_recipients,
            bcc_recipients,
            subject,
            body,
            snippet,
            sent_at,
            is_read,
            thread_root_id,
            reply_to_message_id,
            forwarded_from_message_id,
            created_at,
            updated_at
        "#,
    )
    .bind(message_id)
    .bind(folder.key())
    .fetch_optional(database.pool())
    .await
    .map_err(|source| AppError::Database { source })?
    .ok_or_else(|| AppError::NotFound {
        message: "message not found".to_owned(),
    })
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
