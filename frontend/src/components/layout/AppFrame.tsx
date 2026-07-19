import type { ReactNode } from 'react';

type AppFrameProps = {
  sidebar: ReactNode;
  eyebrow: string;
  title: string;
  children: ReactNode;
};

export function AppFrame({ sidebar, eyebrow, title, children }: AppFrameProps) {
  return (
    <main className="grid min-h-screen bg-canvas text-ink lg:grid-cols-[minmax(13rem,15rem)_minmax(0,1fr)]">
      <aside className="border-b border-line bg-surface px-5 py-6 lg:border-b-0 lg:border-r lg:px-6 lg:py-7">
        <div className="mb-8 text-sm font-semibold text-ink">Workspace</div>
        {sidebar}
      </aside>

      <section className="grid min-w-0 grid-rows-[auto_1fr]" aria-label="Main content">
        <header className="border-b border-line px-6 py-7 lg:px-10">
          <p className="mb-2 text-xs font-semibold uppercase text-ink-muted">{eyebrow}</p>
          <h1 className="text-3xl font-semibold text-ink">{title}</h1>
        </header>
        <div className="min-w-0 px-0">{children}</div>
      </section>
    </main>
  );
}
