CREATE TABLE IF NOT EXISTS folders (
    key TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    sort_order SMALLINT NOT NULL UNIQUE,
    system BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (key IN ('inbox', 'sent', 'drafts', 'archive', 'trash'))
);

INSERT INTO folders (key, display_name, sort_order, system)
VALUES
    ('inbox', 'Inbox', 10, TRUE),
    ('sent', 'Sent', 20, TRUE),
    ('drafts', 'Drafts', 30, TRUE),
    ('archive', 'Archive', 40, TRUE),
    ('trash', 'Trash', 50, TRUE)
ON CONFLICT (key) DO UPDATE SET
    display_name = EXCLUDED.display_name,
    sort_order = EXCLUDED.sort_order,
    system = EXCLUDED.system;

CREATE TABLE IF NOT EXISTS messages (
    id BIGSERIAL PRIMARY KEY,
    seed_key TEXT UNIQUE,
    folder_key TEXT NOT NULL REFERENCES folders(key),
    sender TEXT NOT NULL,
    to_recipients TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    cc_recipients TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    bcc_recipients TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    snippet TEXT NOT NULL,
    sent_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    thread_root_id BIGINT REFERENCES messages(id) ON DELETE SET NULL,
    reply_to_message_id BIGINT REFERENCES messages(id) ON DELETE SET NULL,
    forwarded_from_message_id BIGINT REFERENCES messages(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS messages_folder_sent_at_idx
ON messages (folder_key, sent_at DESC);

CREATE INDEX IF NOT EXISTS messages_folder_read_idx
ON messages (folder_key, is_read);

CREATE INDEX IF NOT EXISTS messages_thread_root_idx
ON messages (thread_root_id);

DROP TRIGGER IF EXISTS messages_set_updated_at ON messages;

CREATE TRIGGER messages_set_updated_at
BEFORE UPDATE ON messages
FOR EACH ROW
EXECUTE FUNCTION set_updated_at_timestamp();

INSERT INTO messages (
    seed_key,
    folder_key,
    sender,
    to_recipients,
    subject,
    body,
    snippet,
    sent_at,
    is_read
)
VALUES
    (
        'sample-inbox-welcome',
        'inbox',
        'hello@example.com',
        ARRAY['inbox@example.com'],
        'A quieter inbox is ready',
        'Your mailbox has been prepared with the core folders and a few starter messages. Future work will connect the list, reading pane, and compose flow.',
        'Your mailbox has been prepared with the core folders and a few starter messages.',
        NOW() - INTERVAL '25 minutes',
        FALSE
    ),
    (
        'sample-inbox-follow-up',
        'inbox',
        'updates@example.com',
        ARRAY['inbox@example.com'],
        'Notes for the first pass',
        'This message gives the first inbox view some real content to scan while the remaining mailbox features are added.',
        'This message gives the first inbox view some real content to scan.',
        NOW() - INTERVAL '2 hours',
        FALSE
    ),
    (
        'sample-inbox-archive-tip',
        'inbox',
        'support@example.com',
        ARRAY['inbox@example.com'],
        'Keeping the mailbox tidy',
        'Archive and Trash are available as fixed folders so move actions can preserve a simple, predictable mailbox model.',
        'Archive and Trash are available as fixed folders for move actions.',
        NOW() - INTERVAL '1 day',
        TRUE
    )
ON CONFLICT (seed_key) DO NOTHING;
