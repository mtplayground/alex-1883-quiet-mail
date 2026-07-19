import './styles.css';

const folders = ['Inbox', 'Sent', 'Drafts', 'Archive', 'Trash'];

export function App() {
  return (
    <main className="app-shell" aria-label="Authenticated mailbox layout">
      <aside className="sidebar" aria-label="Folders">
        <div className="sidebar__brand">Mail</div>
        <nav className="folder-list">
          {folders.map((folder) => (
            <button className="folder-list__item" type="button" key={folder}>
              {folder}
            </button>
          ))}
        </nav>
      </aside>

      <section className="content-area" aria-label="Mailbox content">
        <header className="content-header">
          <p className="eyebrow">Authenticated workspace</p>
          <h1>Inbox</h1>
        </header>
        <div className="empty-state" aria-label="No message selected">
          <p>Select a message to read.</p>
        </div>
      </section>
    </main>
  );
}
