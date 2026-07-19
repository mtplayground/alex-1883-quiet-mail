import { requestJson } from '../api/client';

import type { FoldersResponse, MessageResponse, MessagesResponse } from './types';

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
