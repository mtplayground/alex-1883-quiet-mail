import { AppFrame } from './components/layout/AppFrame';
import { SidebarNav } from './components/layout/SidebarNav';
import { EmptyState } from './components/ui/EmptyState';

const navItems = [
  { label: 'Overview', current: true },
  { label: 'Messages' },
  { label: 'Drafts' },
  { label: 'Archive' },
];

export function App() {
  return (
    <AppFrame
      eyebrow="Authenticated workspace"
      sidebar={<SidebarNav items={navItems} />}
      title="Overview"
    >
      <EmptyState description="The workspace is empty." title="No content selected" />
    </AppFrame>
  );
}
