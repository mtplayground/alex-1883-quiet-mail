import { useMemo, useState, type ReactNode } from 'react';

import { Button } from '../components/ui/Button';
import { classNames } from '../lib/classNames';

import { saveDraft, sendMessage } from './api';
import type { ComposePayload, Message } from './types';

type ComposePanelProps = {
  open: boolean;
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

export function ComposePanel({ open, onClose, onDraftSaved, onSent }: ComposePanelProps) {
  const [form, setForm] = useState(emptyForm);
  const [submitting, setSubmitting] = useState<'draft' | 'send' | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const payload = useMemo<ComposePayload>(
    () => ({
      to: parseRecipients(form.to),
      cc: parseRecipients(form.cc),
      bcc: parseRecipients(form.bcc),
      subject: form.subject,
      body: form.body,
    }),
    [form],
  );
  const canSend =
    payload.to.length > 0 && payload.subject.trim().length > 0 && payload.body.trim().length > 0;

  if (!open) {
    return null;
  }

  async function handleSaveDraft() {
    setSubmitting('draft');
    setStatus(null);

    try {
      const response = await saveDraft(payload);
      onDraftSaved(response.message);
      setStatus('Draft saved.');
    } catch {
      setStatus('Draft could not be saved.');
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
    } catch {
      setStatus('Message could not be sent.');
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
      className="fixed inset-x-3 bottom-3 z-20 rounded-ui border border-line bg-panel shadow-[0_18px_50px_rgb(31_41_51_/_0.16)] sm:left-auto sm:right-5 sm:w-[32rem]"
    >
      <div className="flex items-center justify-between border-b border-line px-4 py-3">
        <h2 className="text-sm font-semibold text-ink">New message</h2>
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
        <p className="min-h-5 text-sm text-ink-muted" role="status">
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
