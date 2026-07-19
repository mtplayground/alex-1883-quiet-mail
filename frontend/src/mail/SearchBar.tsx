import { useState } from 'react';

import { Button } from '../components/ui/Button';
import { classNames } from '../lib/classNames';

type SearchBarProps = {
  activeQuery: string | null;
  loading: boolean;
  onClear: () => void;
  onSearch: (query: string) => void;
};

export function SearchBar({ activeQuery, loading, onClear, onSearch }: SearchBarProps) {
  const [value, setValue] = useState(activeQuery ?? '');

  return (
    <form
      className="flex w-full min-w-0 items-center gap-2 sm:max-w-md"
      onSubmit={(event) => {
        event.preventDefault();
        onSearch(value);
      }}
      role="search"
    >
      <input
        aria-label="Search mail"
        className="min-h-10 min-w-0 flex-1 rounded-ui border border-line bg-panel px-3 text-sm text-ink outline-none transition-colors placeholder:text-ink-soft focus:border-accent focus:ring-2 focus:ring-accent/20"
        onChange={(event) => setValue(event.target.value)}
        placeholder="Search mail"
        type="search"
        value={value}
      />
      {activeQuery ? (
        <Button className="w-auto px-3" onClick={onClear}>
          Clear
        </Button>
      ) : null}
      <Button
        className={classNames('w-auto px-3', loading ? 'cursor-wait opacity-70' : '')}
        disabled={loading}
        type="submit"
        variant="primary"
      >
        {loading ? 'Searching' : 'Search'}
      </Button>
    </form>
  );
}
