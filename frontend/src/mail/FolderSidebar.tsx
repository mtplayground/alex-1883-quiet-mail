import { Button } from '../components/ui/Button';
import { classNames } from '../lib/classNames';

import type { Folder } from './types';

type FolderSidebarProps = {
  folders: Folder[];
  selectedFolder: string;
  loading: boolean;
  onSelectFolder: (folderKey: string) => void;
};

export function FolderSidebar({
  folders,
  selectedFolder,
  loading,
  onSelectFolder,
}: FolderSidebarProps) {
  if (loading && folders.length === 0) {
    return <div className="px-3 text-sm text-ink-muted">Loading folders</div>;
  }

  return (
    <nav className="grid grid-cols-2 gap-1 sm:grid-cols-5 lg:grid-cols-1" aria-label="Folders">
      {folders.map((folder) => {
        const current = folder.key === selectedFolder;

        return (
          <Button
            aria-current={current ? 'page' : undefined}
            className={classNames(
              current ? 'bg-accent-soft text-ink shadow-subtle' : 'text-ink-muted',
              'justify-between',
            )}
            key={folder.key}
            onClick={() => onSelectFolder(folder.key)}
          >
            <span className="truncate">{folder.display_name}</span>
          </Button>
        );
      })}
    </nav>
  );
}
