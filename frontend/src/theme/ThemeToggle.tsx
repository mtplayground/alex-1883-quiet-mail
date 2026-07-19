import { classNames } from '../lib/classNames';

import { useTheme } from './useTheme';

export function ThemeToggle() {
  const { theme, toggleTheme } = useTheme();
  const dark = theme === 'dark';

  return (
    <button
      aria-label={dark ? 'Use light mode' : 'Use dark mode'}
      aria-pressed={dark}
      className="inline-flex min-h-10 items-center gap-2 rounded-ui border border-line bg-panel px-2.5 text-xs font-medium text-ink-muted shadow-subtle transition-colors hover:bg-accent-soft hover:text-ink focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
      onClick={toggleTheme}
      type="button"
    >
      <span
        className={classNames(
          'relative h-5 w-9 rounded-full border border-line transition-colors',
          dark ? 'bg-accent' : 'bg-accent-soft',
        )}
        aria-hidden="true"
      >
        <span
          className={classNames(
            'absolute top-1/2 size-3.5 -translate-y-1/2 rounded-full bg-panel shadow-subtle transition-transform',
            dark ? 'translate-x-[1.125rem]' : 'translate-x-0.5',
          )}
        />
      </span>
      <span>{dark ? 'Dark' : 'Light'}</span>
    </button>
  );
}
