import type { ReactNode } from 'react';

import { LoginPage } from './LoginPage';
import { useAuth } from './useAuth';

type AuthGateProps = {
  children: ReactNode;
};

export function AuthGate({ children }: AuthGateProps) {
  const { error, login, status } = useAuth();

  if (status === 'loading') {
    return (
      <main className="grid min-h-screen place-items-center bg-app px-6 text-sm text-ink-muted">
        Checking session
      </main>
    );
  }

  if (status === 'unauthenticated') {
    return <LoginPage error={error} onLogin={login} />;
  }

  return children;
}
