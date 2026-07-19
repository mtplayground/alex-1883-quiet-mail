import { useMemo, useState, type ReactNode } from 'react';

import { messageFromError } from '../api/client';
import { Button } from '../components/ui/Button';
import { classNames } from '../lib/classNames';

import { saveDraft, sendMessage, updateDraft } from './api';
import type { ComposePayload, Message } from './types';

type ComposePanelProps = {
  open: boolean;
  draft: Message | null;
  onClose: () => void;
  onDraftSaved: (message: Message) => void;
  onSent: (message: Message) => void;
};

const emptyForm = {
  to: '',
  cc: '',
  bcc: '',
  subject: '',
  body: '',
};

const fieldClassName =
  'min-h-10 w-full rounded-ui border border-line bg-surface px-3 text-sm text-ink outline-none transition-colors placeholder:text-ink-soft focus:border-accent disabled:cursor-wait disabled:opacity-60';

export function ComposePanel({ draft, open, onClose, onDraftSaved, onSent }: ComposePanelProps) {
  const [form, setForm] = useState(() => (draft ? formFromMessage(draft) : emptyForm));
  const [submitting, setSubmitting] = useState<'draft' | 'send' | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const payload = useMemo<ComposePayload>(
    () => ({
      to: parseRecipients(form.to),
      cc: parseRecipients(form.cc),
      bcc: parseRecipients(form.bcc),
      subject: form.subject,
      body: form.body,
      thread_root_id: draft?.thread_root_id ?? null,
      reply_to_message_id: draft?.reply_to_message_id ?? null,
      forwarded_from_message_id: draft?.forwarded_from_message_id ?? null,
    }),
    [draft, form],
  );
  const canSend =
    payload.to.length > 0 && payload.subject.trim().length > 0 && payload.body.trim().length > 0;
  const title = draft ? composeTitle(draft) : 'New message';

  if (!open) {
    return null;
  }

  async function handleSaveDraft() {
    setSubmitting('draft');
    setStatus(null);

    try {
      const response = draft ? await updateDraft(draft.id, payload) : await saveDraft(payload);
      onDraftSaved(response.message);
      setStatus('Draft saved.');
    } catch (error) {
      setStatus(messageFromError(error, 'Draft could not be saved. Please try again shortly.'));
    } finally {
      setSubmitting(null);
    }
  }

  async function handleSend() {
    if (!canSend) {
      setStatus('To, subject, and message are required.');
      return;
    }

    setSubmitting('send');
    setStatus(null);

    try {
      const response = await sendMessage(payload);
      onSent(response.message);
      setForm(emptyForm);
      setStatus(null);
      onClose();
    } catch (error) {
      setStatus(messageFromError(error, 'Message could not be sent. Please try again shortly.'));
    } finally {
      setSubmitting(null);
    }
  }

  function handleClose() {
    setStatus(null);
    onClose();
  }

  return (
    <aside
      aria-label="Compose message"
      className="fixed inset-x-3 bottom-3 z-20 rounded-ui border border-line bg-panel/95 shadow-[0_18px_50px_rgb(var(--shadow-subtle)_/_0.18)] backdrop-blur-md sm:left-auto sm:right-5 sm:w-[32rem]"
    >
      <div className="flex items-center justify-between border-b border-line px-4 py-3">
        <h2 className="text-sm font-semibold text-ink">{title}</h2>
        <Button
          aria-label="Close compose"
          className="min-h-8 w-auto px-2 text-xs"
          disabled={submitting !== null}
          onClick={handleClose}
        >
          Close
        </Button>
      </div>

      <div className="grid gap-3 px-4 py-4">
        <Field label="To">
          <input
            className={fieldClassName}
            disabled={submitting !== null}
            onChange={(event) => setForm((current) => ({ ...current, to: event.target.value }))}
            value={form.to}
          />
        </Field>
        <div className="grid gap-3 sm:grid-cols-2">
          <Field label="Cc">
            <input
              className={fieldClassName}
              disabled={submitting !== null}
              onChange={(event) => setForm((current) => ({ ...current, cc: event.target.value }))}
              value={form.cc}
            />
          </Field>
          <Field label="Bcc">
            <input
              className={fieldClassName}
              disabled={submitting !== null}
              onChange={(event) => setForm((current) => ({ ...current, bcc: event.target.value }))}
              value={form.bcc}
            />
          </Field>
        </div>
        <Field label="Subject">
          <input
            className={fieldClassName}
            disabled={submitting !== null}
            onChange={(event) =>
              setForm((current) => ({ ...current, subject: event.target.value }))
            }
            value={form.subject}
          />
        </Field>
        <Field label="Message">
          <textarea
            className={classNames(fieldClassName, 'min-h-44 resize-none py-2 leading-6')}
            disabled={submitting !== null}
            onChange={(event) => setForm((current) => ({ ...current, body: event.target.value }))}
            value={form.body}
          />
        </Field>
      </div>

      <div className="flex flex-col gap-3 border-t border-line px-4 py-3 sm:flex-row sm:items-center sm:justify-between">
        <p className="min-h-5 text-sm text-ink-muted" role="status" aria-live="polite">
          {status}
        </p>
        <div className="flex gap-2">
          <Button
            className="w-auto border border-line bg-panel px-3"
            disabled={submitting !== null}
            onClick={handleSaveDraft}
          >
            {submitting === 'draft' ? 'Saving' : 'Save draft'}
          </Button>
          <Button
            className="w-auto px-4"
            disabled={submitting !== null || !canSend}
            onClick={handleSend}
            variant="primary"
          >
            {submitting === 'send' ? 'Sending' : 'Send'}
          </Button>
        </div>
      </div>
    </aside>
  );
}

function Field({ children, label }: { children: ReactNode; label: string }) {
  return (
    <label className="grid gap-1 text-xs font-medium text-ink-soft">
      <span>{label}</span>
      {children}
    </label>
  );
}

function parseRecipients(value: string) {
  return value
    .split(/[;,]/)
    .map((recipient) => recipient.trim())
    .filter(Boolean);
}

function formFromMessage(message: Message) {
  return {
    to: message.to_recipients.join(', '),
    cc: message.cc_recipients.join(', '),
    bcc: message.bcc_recipients.join(', '),
    subject: message.subject,
    body: message.body,
  };
}

function composeTitle(message: Message) {
  if (message.reply_to_message_id !== null) {
    return 'Reply';
  }

  if (message.forwarded_from_message_id !== null) {
    return 'Forward';
  }

  return 'New message';
}
