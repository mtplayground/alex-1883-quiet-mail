import { requestJson } from '../api/client';

import type { FoldersResponse, MessageResponse, MessagesResponse, MoveAction } from './types';

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
