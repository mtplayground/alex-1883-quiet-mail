import { ApiError, requestJson } from '../api/client';

import type { AuthSession } from './types';

export async function fetchAuthSession(): Promise<AuthSession | null> {
  try {
    return await requestJson<AuthSession>('/api/auth/session');
  } catch (error) {
    if (error instanceof ApiError && error.status === 401) {
      return null;
    }

    throw error;
  }
}

export function redirectToLogin() {
  window.location.assign('/api/auth/login');
}
