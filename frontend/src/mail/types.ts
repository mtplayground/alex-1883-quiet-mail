export type Folder = {
  key: string;
  display_name: string;
  sort_order: number;
};

export type MessageListItem = {
  id: number;
  sender: string;
  subject: string;
  snippet: string;
  sent_at: string;
  is_read: boolean;
};

export type Message = MessageListItem & {
  folder_key: string;
  to_recipients: string[];
  cc_recipients: string[];
  bcc_recipients: string[];
  body: string;
  thread_root_id: number | null;
  reply_to_message_id: number | null;
  forwarded_from_message_id: number | null;
  created_at: string;
  updated_at: string;
};

export type FoldersResponse = {
  folders: Folder[];
};

export type MessagesResponse = {
  folder_key: string;
  messages: MessageListItem[];
};

export type MessageResponse = {
  message: Message;
};

export type EmailDelivery =
  | {
      sent: {
        provider_message_id: string;
      };
    }
  | {
      skipped: {
        reason: string;
      };
    };

export type SendMessageResponse = {
  message: Message;
  delivery: EmailDelivery;
};

export type ComposePayload = {
  to: string[];
  cc: string[];
  bcc: string[];
  subject: string;
  body: string;
};

export type MoveAction = 'archive' | 'trash' | 'restore';
