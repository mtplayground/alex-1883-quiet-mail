import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from 'react';

import type { AuthUser } from '../auth/types';
import { AppFrame } from '../components/layout/AppFrame';
import { Button } from '../components/ui/Button';
import { UserBadge } from '../components/ui/UserBadge';

import { ComposePanel } from './ComposePanel';
import { FolderSidebar } from './FolderSidebar';
import { MessageList } from './MessageList';
import { ReadingPane } from './ReadingPane';
import { SearchBar } from './SearchBar';
import { SearchResultsView } from './SearchResultsView';
import {
  createForwardDraft,
  createReplyDraft,
  fetchFolders,
  fetchMessage,
  fetchMessages,
  moveMessage,
  searchMessages,
} from './api';
import type { Folder, Message, MessageListItem, MoveAction } from './types';

type MailboxContextValue = {
  folders: Folder[];
  foldersLoading: boolean;
  selectedFolder: string;
  selectedFolderName: string;
  selectFolder: (folderKey: string) => void;
  messages: MessageListItem[];
  messagesLoading: boolean;
  messagesError: string | null;
  searchQuery: string | null;
  searchResults: MessageListItem[];
  searchLoading: boolean;
  searchError: string | null;
  selectedMessageId: number | null;
  selectedMessage: Message | null;
  detailLoading: boolean;
  detailError: string | null;
  moveLoading: boolean;
  moveError: string | null;
  replyForwardLoading: boolean;
  replyForwardError: string | null;
  selectMessage: (messageId: number) => void;
  moveSelectedMessage: (action: MoveAction) => void;
  runSearch: (query: string) => void;
  clearSearch: () => void;
  composeOpen: boolean;
  composeDraft: Message | null;
  composeSession: number;
  openCompose: () => void;
  closeCompose: () => void;
  handleComposedMessage: (message: Message) => void;
  createReply: () => void;
  createForward: () => void;
};

const MailboxContext = createContext<MailboxContextValue | undefined>(undefined);

export function MailboxProvider({ children }: { children: ReactNode }) {
  const [folders, setFolders] = useState<Folder[]>([]);
  const [foldersLoading, setFoldersLoading] = useState(true);
  const [selectedFolder, setSelectedFolder] = useState('inbox');
  const [messages, setMessages] = useState<MessageListItem[]>([]);
  const [messagesLoading, setMessagesLoading] = useState(true);
  const [messagesError, setMessagesError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState<string | null>(null);
  const [searchResults, setSearchResults] = useState<MessageListItem[]>([]);
  const [searchLoading, setSearchLoading] = useState(false);
  const [searchError, setSearchError] = useState<string | null>(null);
  const [selectedMessageId, setSelectedMessageId] = useState<number | null>(null);
  const [selectedMessage, setSelectedMessage] = useState<Message | null>(null);
  const [detailLoading, setDetailLoading] = useState(false);
  const [detailError, setDetailError] = useState<string | null>(null);
  const [moveLoading, setMoveLoading] = useState(false);
  const [moveError, setMoveError] = useState<string | null>(null);
  const [replyForwardLoading, setReplyForwardLoading] = useState(false);
  const [replyForwardError, setReplyForwardError] = useState<string | null>(null);
  const [composeOpen, setComposeOpen] = useState(false);
  const [composeDraft, setComposeDraft] = useState<Message | null>(null);
  const [composeSession, setComposeSession] = useState(0);
  const searchRequestRef = useRef(0);

  useEffect(() => {
    let active = true;

    fetchFolders()
      .then((response) => {
        if (!active) {
          return;
        }

        setFolders(response.folders);
      })
      .catch(() => {
        if (!active) {
          return;
        }

        setFolders([]);
      })
      .finally(() => {
        if (active) {
          setFoldersLoading(false);
        }
      });

    return () => {
      active = false;
    };
  }, []);

  useEffect(() => {
    let active = true;

    fetchMessages(selectedFolder)
      .then((response) => {
        if (!active) {
          return;
        }

        setMessages(response.messages);
      })
      .catch(() => {
        if (!active) {
          return;
        }

        setMessages([]);
        setMessagesError('Messages could not be loaded.');
      })
      .finally(() => {
        if (active) {
          setMessagesLoading(false);
        }
      });

    return () => {
      active = false;
    };
  }, [selectedFolder]);

  const selectFolder = useCallback((folderKey: string) => {
    searchRequestRef.current += 1;
    setSearchQuery(null);
    setSearchResults([]);
    setSearchLoading(false);
    setSearchError(null);
    setMessagesLoading(true);
    setMessagesError(null);
    setSelectedMessageId(null);
    setSelectedMessage(null);
    setDetailError(null);
    setMoveError(null);
    setMoveLoading(false);
    setReplyForwardError(null);
    setReplyForwardLoading(false);
    setSelectedFolder(folderKey);
  }, []);

  const selectMessage = useCallback((messageId: number) => {
    setSelectedMessageId(messageId);
    setDetailLoading(true);
    setDetailError(null);
    setMoveError(null);
    setReplyForwardError(null);

    fetchMessage(messageId)
      .then((response) => {
        setSelectedMessage(response.message);
        setMessages((current) =>
          current.map((message) =>
            message.id === messageId ? { ...message, is_read: true } : message,
          ),
        );
        setSearchResults((current) =>
          current.map((message) =>
            message.id === messageId ? { ...message, is_read: true } : message,
          ),
        );
      })
      .catch(() => {
        setSelectedMessage(null);
        setDetailError('Message could not be opened.');
      })
      .finally(() => {
        setDetailLoading(false);
      });
  }, []);

  const moveSelectedMessage = useCallback(
    (action: MoveAction) => {
      if (!selectedMessage) {
        return;
      }

      setMoveLoading(true);
      setMoveError(null);

      moveMessage(selectedMessage.id, action)
        .then((response) => {
          const movedMessage = response.message;

          setSelectedMessage(movedMessage);
          setMessages((current) => {
            if (movedMessage.folder_key !== selectedFolder) {
              return current.filter((message) => message.id !== movedMessage.id);
            }

            return current.map((message) =>
              message.id === movedMessage.id ? toListItem(movedMessage) : message,
            );
          });
          setSearchResults((current) =>
            current.map((message) =>
              message.id === movedMessage.id ? toListItem(movedMessage) : message,
            ),
          );
        })
        .catch(() => {
          setMoveError('Message could not be moved.');
        })
        .finally(() => {
          setMoveLoading(false);
        });
    },
    [selectedFolder, selectedMessage],
  );

  const clearSearch = useCallback(() => {
    searchRequestRef.current += 1;
    setSearchQuery(null);
    setSearchResults([]);
    setSearchLoading(false);
    setSearchError(null);
  }, []);

  const runSearch = useCallback(
    (rawQuery: string) => {
      const query = rawQuery.trim();

      if (!query) {
        clearSearch();
        return;
      }

      const requestId = searchRequestRef.current + 1;
      searchRequestRef.current = requestId;
      setSearchQuery(query);
      setSearchResults([]);
      setSearchLoading(true);
      setSearchError(null);
      setSelectedMessageId(null);
      setSelectedMessage(null);
      setDetailError(null);
      setMoveError(null);
      setReplyForwardError(null);

      searchMessages(query)
        .then((response) => {
          if (searchRequestRef.current !== requestId) {
            return;
          }

          setSearchResults(response.messages);
        })
        .catch(() => {
          if (searchRequestRef.current !== requestId) {
            return;
          }

          setSearchResults([]);
          setSearchError('Search results could not be loaded.');
        })
        .finally(() => {
          if (searchRequestRef.current === requestId) {
            setSearchLoading(false);
          }
        });
    },
    [clearSearch],
  );

  const openCompose = useCallback(() => {
    setComposeDraft(null);
    setComposeSession((current) => current + 1);
    setComposeOpen(true);
  }, []);

  const closeCompose = useCallback(() => {
    setComposeOpen(false);
  }, []);

  const handleComposedMessage = useCallback(
    (message: Message) => {
      if (message.folder_key === 'drafts') {
        setComposeDraft(message);
      } else {
        setComposeDraft((current) => (current?.id === message.id ? message : current));
      }

      if (message.folder_key !== selectedFolder) {
        return;
      }

      setMessages((current) => upsertListItem(current, message));
    },
    [selectedFolder],
  );

  const createThreadedCompose = useCallback(
    (kind: 'reply' | 'forward') => {
      if (!selectedMessage) {
        return;
      }

      setReplyForwardLoading(true);
      setReplyForwardError(null);

      const createDraft = kind === 'reply' ? createReplyDraft : createForwardDraft;

      createDraft(selectedMessage.id)
        .then((response) => {
          const draft = response.message;

          setComposeDraft(draft);
          setComposeOpen(true);
          setMessages((current) => {
            if (selectedFolder !== draft.folder_key) {
              return current;
            }

            return upsertListItem(current, draft);
          });
        })
        .catch(() => {
          setReplyForwardError(
            kind === 'reply' ? 'Reply could not be prepared.' : 'Forward could not be prepared.',
          );
        })
        .finally(() => {
          setReplyForwardLoading(false);
        });
    },
    [selectedFolder, selectedMessage],
  );

  const createReply = useCallback(() => {
    createThreadedCompose('reply');
  }, [createThreadedCompose]);

  const createForward = useCallback(() => {
    createThreadedCompose('forward');
  }, [createThreadedCompose]);

  const selectedFolderName = useMemo(
    () => folders.find((folder) => folder.key === selectedFolder)?.display_name ?? 'Inbox',
    [folders, selectedFolder],
  );

  const value = useMemo<MailboxContextValue>(
    () => ({
      folders,
      foldersLoading,
      selectedFolder,
      selectedFolderName,
      selectFolder,
      messages,
      messagesLoading,
      messagesError,
      searchQuery,
      searchResults,
      searchLoading,
      searchError,
      selectedMessageId,
      selectedMessage,
      detailLoading,
      detailError,
      moveLoading,
      moveError,
      replyForwardLoading,
      replyForwardError,
      selectMessage,
      moveSelectedMessage,
      runSearch,
      clearSearch,
      composeOpen,
      composeDraft,
      composeSession,
      openCompose,
      closeCompose,
      handleComposedMessage,
      createReply,
      createForward,
    }),
    [
      closeCompose,
      clearSearch,
      composeDraft,
      composeOpen,
      composeSession,
      createForward,
      createReply,
      detailError,
      detailLoading,
      folders,
      foldersLoading,
      handleComposedMessage,
      messages,
      messagesError,
      messagesLoading,
      moveError,
      moveLoading,
      moveSelectedMessage,
      openCompose,
      replyForwardError,
      replyForwardLoading,
      runSearch,
      searchError,
      searchLoading,
      searchQuery,
      searchResults,
      selectFolder,
      selectMessage,
      selectedFolder,
      selectedFolderName,
      selectedMessage,
      selectedMessageId,
    ],
  );

  return <MailboxContext.Provider value={value}>{children}</MailboxContext.Provider>;
}

export function MailboxFrame({ user }: { user: AuthUser }) {
  const { clearSearch, runSearch, searchLoading, searchQuery, selectedFolderName } =
    useMailboxContext();

  return (
    <AppFrame
      eyebrow={user.registered ? 'Registration complete' : 'Welcome back'}
      headerAside={
        <div className="flex min-w-0 flex-col gap-3 sm:flex-row sm:items-center sm:justify-end">
          <SearchBar
            activeQuery={searchQuery}
            key={searchQuery ?? 'mailbox-search'}
            loading={searchLoading}
            onClear={clearSearch}
            onSearch={runSearch}
          />
          <div className="shrink-0">
            <UserBadge user={user} />
          </div>
        </div>
      }
      sidebar={<MailboxSidebar />}
      title={searchQuery ? 'Search' : selectedFolderName}
    >
      <MailboxContent />
    </AppFrame>
  );
}

export function MailboxSidebar() {
  const { folders, foldersLoading, selectedFolder, selectFolder } = useMailboxContext();

  return (
    <FolderSidebar
      folders={folders}
      loading={foldersLoading}
      onSelectFolder={selectFolder}
      selectedFolder={selectedFolder}
    />
  );
}

export function MailboxContent() {
  const {
    closeCompose,
    composeDraft,
    composeOpen,
    composeSession,
    createForward,
    createReply,
    detailError,
    detailLoading,
    handleComposedMessage,
    messages,
    messagesError,
    messagesLoading,
    moveError,
    moveLoading,
    moveSelectedMessage,
    replyForwardError,
    replyForwardLoading,
    searchError,
    searchLoading,
    searchQuery,
    searchResults,
    selectedMessage,
    selectedMessageId,
    selectMessage,
  } = useMailboxContext();
  const showingSearchResults = searchQuery !== null;

  return (
    <>
      <section className="grid min-h-[calc(100vh-8.5rem)] min-w-0 lg:grid-cols-[minmax(20rem,28rem)_minmax(0,1fr)]">
        <div className="border-b border-line bg-panel lg:border-b-0 lg:border-r">
          <MessageListHeader />
          {showingSearchResults ? (
            <SearchResultsView
              error={searchError}
              loading={searchLoading}
              messages={searchResults}
              onSelectMessage={selectMessage}
              query={searchQuery}
              selectedMessageId={selectedMessageId}
            />
          ) : (
            <MessageList
              error={messagesError}
              loading={messagesLoading}
              messages={messages}
              onSelectMessage={selectMessage}
              selectedMessageId={selectedMessageId}
            />
          )}
        </div>
        <div className="min-w-0 bg-surface">
          <ReadingPane
            error={detailError}
            loading={detailLoading}
            message={selectedMessage}
            moveError={moveError}
            moveLoading={moveLoading}
            onMoveMessage={moveSelectedMessage}
            onForward={createForward}
            onReply={createReply}
            replyForwardError={replyForwardError}
            replyForwardLoading={replyForwardLoading}
          />
        </div>
      </section>
      <ComposePanel
        draft={composeDraft}
        key={composeDraft?.id ?? `new-${composeSession}`}
        onClose={closeCompose}
        onDraftSaved={handleComposedMessage}
        onSent={handleComposedMessage}
        open={composeOpen}
      />
    </>
  );
}

function MessageListHeader() {
  const { openCompose, searchQuery, selectedFolderName } = useMailboxContext();
  const title = searchQuery ? `Search: ${searchQuery}` : selectedFolderName;

  return (
    <div className="flex items-center justify-between gap-3 border-b border-line px-5 py-4 lg:px-6">
      <p className="min-w-0 truncate text-sm font-semibold text-ink">{title}</p>
      <Button className="w-auto px-3" onClick={openCompose} variant="primary">
        Compose
      </Button>
    </div>
  );
}

function useMailboxContext() {
  const context = useContext(MailboxContext);

  if (!context) {
    throw new Error('Mailbox context is missing.');
  }

  return context;
}

function toListItem(message: Message): MessageListItem {
  return {
    id: message.id,
    sender: message.sender,
    subject: message.subject,
    snippet: message.snippet,
    sent_at: message.sent_at,
    is_read: message.is_read,
  };
}

function upsertListItem(messages: MessageListItem[], message: Message) {
  const listItem = toListItem(message);
  const exists = messages.some((current) => current.id === message.id);

  if (!exists) {
    return [listItem, ...messages];
  }

  return messages.map((current) => (current.id === message.id ? listItem : current));
}
