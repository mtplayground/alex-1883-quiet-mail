#![allow(dead_code)]

use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put},
    Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    auth::AuthenticatedUser,
    db::Database,
    email::{EmailDelivery, OutboundEmail},
    error::AppError,
};

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
pub struct SearchMessagesResponse {
    pub query: String,
    pub messages: Vec<MessageListItem>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: Message,
}

#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub message: Message,
    pub delivery: EmailDelivery,
}

#[derive(Debug, Deserialize)]
pub struct ComposeMessageRequest {
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub thread_root_id: Option<i64>,
    pub reply_to_message_id: Option<i64>,
    pub forwarded_from_message_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SaveDraftRequest {
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct MoveMessageRequest {
    pub action: MoveAction,
}

#[derive(Debug, Deserialize)]
pub struct SearchMessagesQuery {
    pub q: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/folders", get(list_folders))
        .route("/folders/:folder_key/messages", get(list_messages))
        .route("/search", get(search_messages))
        .route("/compose/send", post(send_message))
        .route("/drafts", get(list_drafts).post(save_draft))
        .route("/drafts/:message_id", put(update_draft))
        .route("/messages/:message_id", get(message_detail))
        .route("/messages/:message_id/reply", post(create_reply_draft))
        .route("/messages/:message_id/forward", post(create_forward_draft))
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

async fn search_messages(
    State(state): State<AppState>,
    Query(query): Query<SearchMessagesQuery>,
) -> Result<Json<SearchMessagesResponse>, AppError> {
    let search = query.normalized()?;
    let messages = search_messages_by_query(&state.database, &search).await?;

    Ok(Json(SearchMessagesResponse {
        query: search,
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

async fn send_message(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<ComposeMessageRequest>,
) -> Result<Json<SendMessageResponse>, AppError> {
    request.validate_for_send()?;
    let sent = state
        .email
        .send_and_record(&state.database, request.into_outbound_email(user.email))
        .await?;

    Ok(Json(SendMessageResponse {
        message: sent.message,
        delivery: sent.delivery,
    }))
}

async fn list_drafts(State(state): State<AppState>) -> Result<Json<MessagesResponse>, AppError> {
    let messages = fetch_messages_for_folder(&state.database, SystemFolder::Drafts).await?;

    Ok(Json(MessagesResponse {
        folder_key: SystemFolder::Drafts.key().to_owned(),
        messages,
    }))
}

async fn save_draft(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<SaveDraftRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let message = insert_draft_message(&state.database, request.into_draft(user.email)).await?;

    Ok(Json(MessageResponse { message }))
}

async fn update_draft(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(message_id): Path<i64>,
    Json(request): Json<SaveDraftRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    let message =
        update_draft_message(&state.database, message_id, request.into_draft(user.email)).await?;

    Ok(Json(MessageResponse { message }))
}

async fn create_reply_draft(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(message_id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let message = create_threaded_draft(
        &state.database,
        message_id,
        ThreadedDraftKind::Reply,
        user.email,
    )
    .await?;

    Ok(Json(MessageResponse { message }))
}

async fn create_forward_draft(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(message_id): Path<i64>,
) -> Result<Json<MessageResponse>, AppError> {
    let message = create_threaded_draft(
        &state.database,
        message_id,
        ThreadedDraftKind::Forward,
        user.email,
    )
    .await?;

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

impl ComposeMessageRequest {
    fn validate_for_send(&self) -> Result<(), AppError> {
        if normalize_recipients(self.to.iter()).is_empty() {
            return Err(AppError::BadRequest {
                message: "at least one recipient is required".to_owned(),
            });
        }

        if self.subject.trim().is_empty() {
            return Err(AppError::BadRequest {
                message: "subject is required".to_owned(),
            });
        }

        if self.body.trim().is_empty() {
            return Err(AppError::BadRequest {
                message: "message body is required".to_owned(),
            });
        }

        Ok(())
    }

    fn into_outbound_email(self, sender: String) -> OutboundEmail {
        OutboundEmail {
            sender,
            to: normalize_recipients(self.to.iter()),
            cc: normalize_recipients(self.cc.iter()),
            bcc: normalize_recipients(self.bcc.iter()),
            subject: self.subject.trim().to_owned(),
            html: None,
            text: Some(self.body),
            reply_to: None,
            thread_root_id: self.thread_root_id,
            reply_to_message_id: self.reply_to_message_id,
            forwarded_from_message_id: self.forwarded_from_message_id,
        }
    }
}

impl SaveDraftRequest {
    fn into_draft(self, sender: String) -> DraftMessageUpsert {
        let body = self.body;
        let snippet = snippet_from_body(&body);

        DraftMessageUpsert {
            sender,
            to_recipients: normalize_recipients(self.to.iter()),
            cc_recipients: normalize_recipients(self.cc.iter()),
            bcc_recipients: normalize_recipients(self.bcc.iter()),
            subject: self.subject.trim().to_owned(),
            body,
            snippet,
            thread_root_id: None,
            reply_to_message_id: None,
            forwarded_from_message_id: None,
        }
    }
}

impl SearchMessagesQuery {
    fn normalized(self) -> Result<String, AppError> {
        let query = self.q.trim();

        if query.is_empty() {
            return Err(AppError::BadRequest {
                message: "search query is required".to_owned(),
            });
        }

        if query.chars().count() > 200 {
            return Err(AppError::BadRequest {
                message: "search query must be 200 characters or fewer".to_owned(),
            });
        }

        Ok(query.to_owned())
    }
}

#[derive(Debug, Clone, Copy)]
enum ThreadedDraftKind {
    Reply,
    Forward,
}

impl ThreadedDraftKind {
    fn subject(self, source: &Message) -> String {
        let prefix = match self {
            Self::Reply => "Re:",
            Self::Forward => "Fwd:",
        };

        if source
            .subject
            .get(..prefix.len())
            .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
        {
            source.subject.clone()
        } else {
            format!("{prefix} {}", source.subject)
        }
    }

    fn body(self, source: &Message) -> String {
        match self {
            Self::Reply => quoted_reply_body(source),
            Self::Forward => forwarded_body(source),
        }
    }

    fn to_recipients(self, source: &Message) -> Vec<String> {
        match self {
            Self::Reply => vec![source.sender.clone()],
            Self::Forward => Vec::new(),
        }
    }
}

fn quoted_reply_body(source: &Message) -> String {
    let quoted = source
        .body
        .lines()
        .map(|line| format!("> {line}"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "\n\nOn {}, {} wrote:\n{}",
        source.sent_at.to_rfc3339(),
        source.sender,
        quoted
    )
}

fn forwarded_body(source: &Message) -> String {
    let to = source.to_recipients.join(", ");

    format!(
        "\n\nForwarded message\nFrom: {}\nTo: {}\nDate: {}\nSubject: {}\n\n{}",
        source.sender,
        to,
        source.sent_at.to_rfc3339(),
        source.subject,
        source.body
    )
}

fn normalize_recipients<'a>(recipients: impl IntoIterator<Item = &'a String>) -> Vec<String> {
    recipients
        .into_iter()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .collect()
}

pub fn snippet_from_body(body: &str) -> String {
    let normalized = body.split_whitespace().collect::<Vec<_>>().join(" ");
    normalized.chars().take(160).collect()
}

fn escaped_like_pattern(value: &str) -> String {
    let mut pattern = String::with_capacity(value.len() + 2);
    pattern.push('%');

    for character in value.to_lowercase().chars() {
        match character {
            '%' | '_' | '\\' => {
                pattern.push('\\');
                pattern.push(character);
            }
            _ => pattern.push(character),
        }
    }

    pattern.push('%');
    pattern
}

async fn create_threaded_draft(
    database: &Database,
    source_message_id: i64,
    kind: ThreadedDraftKind,
    sender: String,
) -> Result<Message, AppError> {
    let source = fetch_message_by_id(database, source_message_id).await?;
    let body = kind.body(&source);
    let snippet = snippet_from_body(&body);
    let thread_root_id = Some(source.thread_root_id.unwrap_or(source.id));

    insert_draft_message(
        database,
        DraftMessageUpsert {
            sender,
            to_recipients: kind.to_recipients(&source),
            cc_recipients: Vec::new(),
            bcc_recipients: Vec::new(),
            subject: kind.subject(&source),
            body,
            snippet,
            thread_root_id,
            reply_to_message_id: match kind {
                ThreadedDraftKind::Reply => Some(source.id),
                ThreadedDraftKind::Forward => None,
            },
            forwarded_from_message_id: match kind {
                ThreadedDraftKind::Reply => None,
                ThreadedDraftKind::Forward => Some(source.id),
            },
        },
    )
    .await
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

pub async fn fetch_message_by_id(
    database: &Database,
    message_id: i64,
) -> Result<Message, AppError> {
    sqlx::query_as::<_, Message>(
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
    .fetch_optional(database.pool())
    .await
    .map_err(|source| AppError::Database { source })?
    .ok_or_else(|| AppError::NotFound {
        message: "message not found".to_owned(),
    })
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

pub struct SentMessageInsert {
    pub sender: String,
    pub to_recipients: Vec<String>,
    pub cc_recipients: Vec<String>,
    pub bcc_recipients: Vec<String>,
    pub subject: String,
    pub body: String,
    pub snippet: String,
    pub thread_root_id: Option<i64>,
    pub reply_to_message_id: Option<i64>,
    pub forwarded_from_message_id: Option<i64>,
}

pub struct DraftMessageUpsert {
    pub sender: String,
    pub to_recipients: Vec<String>,
    pub cc_recipients: Vec<String>,
    pub bcc_recipients: Vec<String>,
    pub subject: String,
    pub body: String,
    pub snippet: String,
    pub thread_root_id: Option<i64>,
    pub reply_to_message_id: Option<i64>,
    pub forwarded_from_message_id: Option<i64>,
}

pub async fn insert_sent_message(
    database: &Database,
    message: SentMessageInsert,
) -> Result<Message, AppError> {
    sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (
            folder_key,
            sender,
            to_recipients,
            cc_recipients,
            bcc_recipients,
            subject,
            body,
            snippet,
            thread_root_id,
            reply_to_message_id,
            forwarded_from_message_id,
            sent_at,
            is_read
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), TRUE)
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
    .bind(SystemFolder::Sent.key())
    .bind(message.sender)
    .bind(message.to_recipients)
    .bind(message.cc_recipients)
    .bind(message.bcc_recipients)
    .bind(message.subject)
    .bind(message.body)
    .bind(message.snippet)
    .bind(message.thread_root_id)
    .bind(message.reply_to_message_id)
    .bind(message.forwarded_from_message_id)
    .fetch_one(database.pool())
    .await
    .map_err(|source| AppError::Database { source })
}

pub async fn insert_draft_message(
    database: &Database,
    message: DraftMessageUpsert,
) -> Result<Message, AppError> {
    sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (
            folder_key,
            sender,
            to_recipients,
            cc_recipients,
            bcc_recipients,
            subject,
            body,
            snippet,
            thread_root_id,
            reply_to_message_id,
            forwarded_from_message_id,
            sent_at,
            is_read
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW(), TRUE)
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
    .bind(SystemFolder::Drafts.key())
    .bind(message.sender)
    .bind(message.to_recipients)
    .bind(message.cc_recipients)
    .bind(message.bcc_recipients)
    .bind(message.subject)
    .bind(message.body)
    .bind(message.snippet)
    .bind(message.thread_root_id)
    .bind(message.reply_to_message_id)
    .bind(message.forwarded_from_message_id)
    .fetch_one(database.pool())
    .await
    .map_err(|source| AppError::Database { source })
}

pub async fn update_draft_message(
    database: &Database,
    message_id: i64,
    message: DraftMessageUpsert,
) -> Result<Message, AppError> {
    sqlx::query_as::<_, Message>(
        r#"
        UPDATE messages
        SET
            sender = $2,
            to_recipients = $3,
            cc_recipients = $4,
            bcc_recipients = $5,
            subject = $6,
            body = $7,
            snippet = $8,
            sent_at = NOW(),
            is_read = TRUE
        WHERE id = $1 AND folder_key = $9
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
    .bind(message.sender)
    .bind(message.to_recipients)
    .bind(message.cc_recipients)
    .bind(message.bcc_recipients)
    .bind(message.subject)
    .bind(message.body)
    .bind(message.snippet)
    .bind(SystemFolder::Drafts.key())
    .fetch_optional(database.pool())
    .await
    .map_err(|source| AppError::Database { source })?
    .ok_or_else(|| AppError::NotFound {
        message: "draft not found".to_owned(),
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

pub async fn search_messages_by_query(
    database: &Database,
    query: &str,
) -> Result<Vec<MessageListItem>, AppError> {
    let like_pattern = escaped_like_pattern(query);

    sqlx::query_as::<_, MessageListItem>(
        r#"
        WITH search AS (
            SELECT
                websearch_to_tsquery('simple', $1) AS query,
                $2::TEXT AS pattern
        )
        SELECT id, sender, subject, snippet, sent_at, is_read
        FROM messages, search
        WHERE
            to_tsvector('simple', subject || ' ' || sender || ' ' || body) @@ search.query
            OR lower(subject) LIKE search.pattern ESCAPE '\'
            OR lower(sender) LIKE search.pattern ESCAPE '\'
            OR lower(body) LIKE search.pattern ESCAPE '\'
        ORDER BY
            (
                ts_rank_cd(
                    setweight(to_tsvector('simple', subject), 'A') ||
                    setweight(to_tsvector('simple', sender), 'B') ||
                    setweight(to_tsvector('simple', body), 'C'),
                    search.query
                )
                + CASE WHEN lower(subject) LIKE search.pattern ESCAPE '\' THEN 0.30 ELSE 0 END
                + CASE WHEN lower(sender) LIKE search.pattern ESCAPE '\' THEN 0.20 ELSE 0 END
                + CASE WHEN lower(body) LIKE search.pattern ESCAPE '\' THEN 0.10 ELSE 0 END
            ) DESC,
            sent_at DESC,
            id DESC
        LIMIT 50
        "#,
    )
    .bind(query)
    .bind(like_pattern)
    .fetch_all(database.pool())
    .await
    .map_err(|source| AppError::Database { source })
}
