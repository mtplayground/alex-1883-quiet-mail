import { AuthGate } from './auth/AuthGate';
import { AuthProvider } from './auth/AuthContext';
import { useAuth } from './auth/useAuth';
import { MailboxFrame, MailboxProvider } from './mail/MailboxView';

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
    <MailboxProvider>
      <MailboxFrame user={user} />
    </MailboxProvider>
  );
}
