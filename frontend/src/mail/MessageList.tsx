import { classNames } from '../lib/classNames';

import type { MessageListItem } from './types';

type MessageListProps = {
  messages: MessageListItem[];
  selectedMessageId: number | null;
  loading: boolean;
  error: string | null;
  onSelectMessage: (messageId: number) => void;
};

const listDateFormatter = new Intl.DateTimeFormat(undefined, {
  month: 'short',
  day: 'numeric',
});

export function MessageList({
  messages,
  selectedMessageId,
  loading,
  error,
  onSelectMessage,
}: MessageListProps) {
  if (loading && messages.length === 0) {
    return <ListStatus label="Loading messages" />;
  }

  if (error) {
    return <ListStatus label={error} />;
  }

  if (messages.length === 0) {
    return <ListStatus label="No messages in this folder" />;
  }

  return (
    <div className="divide-y divide-line" role="list" aria-label="Messages">
      {messages.map((message) => {
        const selected = selectedMessageId === message.id;

        return (
          <button
            aria-current={selected ? 'true' : undefined}
            className={classNames(
              'grid w-full grid-cols-[auto_minmax(0,1fr)_auto] gap-3 px-5 py-4 text-left transition-colors hover:bg-accent-soft focus-visible:outline focus-visible:outline-2 focus-visible:outline-inset focus-visible:outline-accent lg:px-6',
              selected ? 'bg-accent-soft' : 'bg-transparent',
            )}
            key={message.id}
            onClick={() => onSelectMessage(message.id)}
            type="button"
          >
            <span
              className={classNames(
                'mt-1.5 size-2 rounded-full',
                message.is_read ? 'bg-transparent' : 'bg-accent',
              )}
              aria-hidden="true"
            />
            <span className="min-w-0">
              <span
                className={classNames(
                  'block truncate text-sm',
                  message.is_read ? 'font-medium text-ink' : 'font-semibold text-ink',
                )}
              >
                {message.sender}
              </span>
              <span
                className={classNames(
                  'mt-1 block truncate text-sm',
                  message.is_read ? 'font-medium text-ink-muted' : 'font-semibold text-ink',
                )}
              >
                {message.subject}
              </span>
              <span className="mt-1 block truncate text-sm text-ink-muted">{message.snippet}</span>
            </span>
            <time className="pt-0.5 text-xs text-ink-soft" dateTime={message.sent_at}>
              {formatDate(message.sent_at, listDateFormatter)}
            </time>
          </button>
        );
      })}
    </div>
  );
}

function ListStatus({ label }: { label: string }) {
  return <div className="px-5 py-8 text-sm text-ink-muted lg:px-6">{label}</div>;
}

function formatDate(value: string, formatter: Intl.DateTimeFormat) {
  const date = new Date(value);

  if (Number.isNaN(date.valueOf())) {
    return value;
  }

  return formatter.format(date);
}
