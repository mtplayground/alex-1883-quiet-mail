import { Button } from '../components/ui/Button';
import { classNames } from '../lib/classNames';

import type { Message, MoveAction } from './types';

type ReadingPaneProps = {
  message: Message | null;
  loading: boolean;
  error: string | null;
  moveLoading: boolean;
  moveError: string | null;
  onMoveMessage: (action: MoveAction) => void;
};

const detailDateFormatter = new Intl.DateTimeFormat(undefined, {
  month: 'short',
  day: 'numeric',
  hour: 'numeric',
  minute: '2-digit',
});

export function ReadingPane({
  message,
  loading,
  error,
  moveLoading,
  moveError,
  onMoveMessage,
}: ReadingPaneProps) {
  if (loading) {
    return <PaneStatus label="Opening message" />;
  }

  if (error) {
    return <PaneStatus label={error} />;
  }

  if (!message) {
    return <PaneStatus label="Select a message" />;
  }

  return (
    <article className="min-h-full px-6 py-6 lg:px-8">
      <div className="flex flex-col gap-5 border-b border-line pb-5 xl:flex-row xl:items-start xl:justify-between">
        <div className="min-w-0">
          <p className="text-sm text-ink-muted">{message.sender}</p>
          <h2 className="mt-2 text-xl font-semibold leading-8 text-ink">{message.subject}</h2>
          <time className="mt-2 block text-xs text-ink-soft" dateTime={message.sent_at}>
            {formatDate(message.sent_at)}
          </time>
        </div>
        <MoveActions folderKey={message.folder_key} loading={moveLoading} onMove={onMoveMessage} />
      </div>

      <dl className="grid gap-3 border-b border-line py-5 text-sm sm:grid-cols-[5rem_minmax(0,1fr)]">
        <MetadataRow label="From" value={message.sender} />
        <MetadataRow label="To" value={formatRecipients(message.to_recipients)} />
        {message.cc_recipients.length > 0 ? (
          <MetadataRow label="Cc" value={formatRecipients(message.cc_recipients)} />
        ) : null}
        {message.bcc_recipients.length > 0 ? (
          <MetadataRow label="Bcc" value={formatRecipients(message.bcc_recipients)} />
        ) : null}
        <MetadataRow label="Folder" value={folderLabel(message.folder_key)} />
      </dl>

      {moveError ? <p className="pt-5 text-sm text-ink-muted">{moveError}</p> : null}

      <p className="whitespace-pre-wrap py-6 text-sm leading-7 text-ink">{message.body}</p>
    </article>
  );
}

function MoveActions({
  folderKey,
  loading,
  onMove,
}: {
  folderKey: string;
  loading: boolean;
  onMove: (action: MoveAction) => void;
}) {
  const actions = moveActionsForFolder(folderKey);

  return (
    <div className="flex flex-wrap gap-2">
      {actions.map((action) => (
        <Button
          className={classNames(
            'min-h-9 w-auto border border-line bg-panel px-3 text-xs',
            loading ? 'cursor-wait opacity-60' : '',
          )}
          disabled={loading}
          key={action}
          onClick={() => onMove(action)}
        >
          {moveLabel(action)}
        </Button>
      ))}
    </div>
  );
}

function MetadataRow({ label, value }: { label: string; value: string }) {
  return (
    <>
      <dt className="text-ink-soft">{label}</dt>
      <dd className="min-w-0 break-words text-ink-muted">{value}</dd>
    </>
  );
}

function PaneStatus({ label }: { label: string }) {
  return (
    <div className="grid min-h-72 place-items-center px-6 text-sm text-ink-muted">{label}</div>
  );
}

function moveActionsForFolder(folderKey: string): MoveAction[] {
  if (folderKey === 'trash') {
    return ['restore'];
  }

  if (folderKey === 'archive') {
    return ['restore', 'trash'];
  }

  return ['archive', 'trash'];
}

function moveLabel(action: MoveAction) {
  switch (action) {
    case 'archive':
      return 'Archive';
    case 'trash':
      return 'Trash';
    case 'restore':
      return 'Restore';
  }
}

function folderLabel(folderKey: string) {
  switch (folderKey) {
    case 'inbox':
      return 'Inbox';
    case 'sent':
      return 'Sent';
    case 'drafts':
      return 'Drafts';
    case 'archive':
      return 'Archive';
    case 'trash':
      return 'Trash';
    default:
      return folderKey;
  }
}

function formatRecipients(recipients: string[]) {
  return recipients.length > 0 ? recipients.join(', ') : 'None';
}

function formatDate(value: string) {
  const date = new Date(value);

  if (Number.isNaN(date.valueOf())) {
    return value;
  }

  return detailDateFormatter.format(date);
}
