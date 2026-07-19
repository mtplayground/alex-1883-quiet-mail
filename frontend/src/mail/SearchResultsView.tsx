import { MessageList } from './MessageList';
import type { MessageListItem } from './types';

type SearchResultsViewProps = {
  error: string | null;
  loading: boolean;
  messages: MessageListItem[];
  onSelectMessage: (messageId: number) => void;
  query: string;
  selectedMessageId: number | null;
};

export function SearchResultsView({
  error,
  loading,
  messages,
  onSelectMessage,
  query,
  selectedMessageId,
}: SearchResultsViewProps) {
  return (
    <MessageList
      ariaLabel="Search results"
      emptyDescription={`No messages matched "${query}".`}
      emptyTitle="No results"
      error={error}
      errorTitle="Search unavailable"
      loading={loading}
      loadingLabel="Searching messages"
      messages={messages}
      onSelectMessage={onSelectMessage}
      selectedMessageId={selectedMessageId}
    />
  );
}
