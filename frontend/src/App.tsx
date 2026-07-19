import { AuthGate } from './auth/AuthGate';
import { AuthProvider } from './auth/AuthContext';
import { useAuth } from './auth/useAuth';
import { MailboxFrame, MailboxProvider } from './mail/MailboxView';
import { ThemeProvider } from './theme/ThemeContext';

export function App() {
  return (
    <ThemeProvider>
      <AuthProvider>
        <AuthGate>
          <AuthenticatedApp />
        </AuthGate>
      </AuthProvider>
    </ThemeProvider>
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
