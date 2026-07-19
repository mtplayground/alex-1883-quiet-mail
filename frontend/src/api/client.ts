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
    throw new ApiError(`Request failed with status ${response.status}`, response.status);
  }

  return response.json() as Promise<T>;
}
