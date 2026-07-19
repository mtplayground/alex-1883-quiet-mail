export class ApiError extends Error {
  constructor(
    message: string,
    public readonly status: number,
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

type RequestOptions = RequestInit & {
  expectedStatuses?: number[];
};

type ApiErrorBody = {
  message?: unknown;
  error?: unknown;
};

export async function requestJson<T>(path: string, options: RequestOptions = {}): Promise<T> {
  const { expectedStatuses = [200], headers, ...requestOptions } = options;
  const response = await fetch(path, {
    credentials: 'include',
    headers: {
      Accept: 'application/json',
      ...headers,
    },
    ...requestOptions,
  });

  if (!expectedStatuses.includes(response.status)) {
    throw new ApiError(await readErrorMessage(response), response.status);
  }

  return response.json() as Promise<T>;
}

export function messageFromError(error: unknown, fallback: string) {
  if (error instanceof ApiError && error.message) {
    return error.message;
  }

  return fallback;
}

async function readErrorMessage(response: Response) {
  const fallback = `Request failed with status ${response.status}`;
  const contentType = response.headers.get('content-type') ?? '';

  try {
    if (contentType.includes('application/json')) {
      const body = (await response.json()) as ApiErrorBody;

      if (typeof body.message === 'string' && body.message.trim()) {
        return body.message;
      }

      if (typeof body.error === 'string' && body.error.trim()) {
        return body.error;
      }

      return fallback;
    }

    const text = await response.text();
    return text.trim() || fallback;
  } catch {
    return fallback;
  }
}
