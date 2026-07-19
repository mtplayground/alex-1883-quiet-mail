import { requestJson } from '../api/client';

import type {
  ComposePayload,
  FoldersResponse,
  MessageResponse,
  MessagesResponse,
  MoveAction,
  SendMessageResponse,
} from './types';

export function fetchFolders() {
  return requestJson<FoldersResponse>('/api/mailbox/folders');
}

export function fetchMessages(folderKey: string) {
  return requestJson<MessagesResponse>(
    `/api/mailbox/folders/${encodeURIComponent(folderKey)}/messages`,
  );
}

export function fetchMessage(messageId: number) {
  return requestJson<MessageResponse>(`/api/mailbox/messages/${messageId}`);
}

export function moveMessage(messageId: number, action: MoveAction) {
  return requestJson<MessageResponse>(`/api/mailbox/messages/${messageId}/move`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ action }),
  });
}

export function sendMessage(payload: ComposePayload) {
  return requestJson<SendMessageResponse>('/api/mailbox/compose/send', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });
}

export function saveDraft(payload: ComposePayload) {
  return requestJson<MessageResponse>('/api/mailbox/drafts', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });
}

export function updateDraft(messageId: number, payload: ComposePayload) {
  return requestJson<MessageResponse>(`/api/mailbox/drafts/${messageId}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });
}

export function createReplyDraft(messageId: number) {
  return requestJson<MessageResponse>(`/api/mailbox/messages/${messageId}/reply`, {
    method: 'POST',
  });
}

export function createForwardDraft(messageId: number) {
  return requestJson<MessageResponse>(`/api/mailbox/messages/${messageId}/forward`, {
    method: 'POST',
  });
}
