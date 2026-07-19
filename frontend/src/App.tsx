import { AuthGate } from './auth/AuthGate';
import { AuthProvider } from './auth/AuthContext';
import { useAuth } from './auth/useAuth';
import { AppFrame } from './components/layout/AppFrame';
import { SidebarNav } from './components/layout/SidebarNav';
import { EmptyState } from './components/ui/EmptyState';
import { UserBadge } from './components/ui/UserBadge';

const navItems = [
  { label: 'Overview', current: true },
  { label: 'Messages' },
  { label: 'Drafts' },
  { label: 'Archive' },
];

export function App() {
  return (
    <AuthProvider>
      <AuthGate>
        <AuthenticatedApp />
      </AuthGate>
    </AuthProvider>
  );
}

function AuthenticatedApp() {
  const { user } = useAuth();

  if (!user) {
    return null;
  }

  return (
    <AppFrame
      eyebrow={user.registered ? 'Registration complete' : 'Welcome back'}
      headerAside={<UserBadge user={user} />}
      sidebar={<SidebarNav items={navItems} />}
      title="Inbox"
    >
      <EmptyState description="No messages to show." title="No messages yet" />
    </AppFrame>
  );
}
