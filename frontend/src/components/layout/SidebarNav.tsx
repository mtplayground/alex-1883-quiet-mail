import { Button } from '../ui/Button';

export type SidebarNavItem = {
  label: string;
  current?: boolean;
};

type SidebarNavProps = {
  items: SidebarNavItem[];
};

export function SidebarNav({ items }: SidebarNavProps) {
  return (
    <nav
      className="grid grid-cols-2 gap-1 sm:grid-cols-4 lg:grid-cols-1"
      aria-label="Primary navigation"
    >
      {items.map((item) => (
        <Button
          aria-current={item.current ? 'page' : undefined}
          className={item.current ? 'bg-accent-soft text-ink shadow-subtle' : 'text-ink-muted'}
          key={item.label}
        >
          {item.label}
        </Button>
      ))}
    </nav>
  );
}
