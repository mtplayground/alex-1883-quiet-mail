import { createContext, useCallback, useEffect, useMemo, useState, type ReactNode } from 'react';

import { fetchAuthSession, redirectToLogin } from './api';
import type { AuthSession, AuthUser } from './types';

type AuthStatus = 'loading' | 'authenticated' | 'unauthenticated';

type AuthContextValue = {
  status: AuthStatus;
  user: AuthUser | null;
  error: string | null;
  refresh: () => Promise<void>;
  login: () => void;
};

const AuthContext = createContext<AuthContextValue | undefined>(undefined);
export { AuthContext };

type AuthProviderProps = {
  children: ReactNode;
};

export function AuthProvider({ children }: AuthProviderProps) {
  const [session, setSession] = useState<AuthSession | null>(null);
  const [status, setStatus] = useState<AuthStatus>('loading');
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setStatus('loading');
    setError(null);

    try {
      const nextSession = await fetchAuthSession();
      setSession(nextSession);
      setStatus(nextSession ? 'authenticated' : 'unauthenticated');
    } catch {
      setSession(null);
      setStatus('unauthenticated');
      setError('Session check failed.');
    }
  }, []);

  useEffect(() => {
    let active = true;

    fetchAuthSession()
      .then((nextSession) => {
        if (!active) {
          return;
        }

        setSession(nextSession);
        setStatus(nextSession ? 'authenticated' : 'unauthenticated');
      })
      .catch(() => {
        if (!active) {
          return;
        }

        setSession(null);
        setStatus('unauthenticated');
        setError('Session check failed.');
      });

    return () => {
      active = false;
    };
  }, []);

  const value = useMemo<AuthContextValue>(
    () => ({
      status,
      user: session?.user ?? null,
      error,
      refresh,
      login: redirectToLogin,
    }),
    [error, refresh, session?.user, status],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
