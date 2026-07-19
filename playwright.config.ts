import { defineConfig } from '@playwright/test';

const port = Number(process.env.E2E_PORT ?? 8080);
const baseURL = process.env.E2E_BASE_URL ?? `http://127.0.0.1:${port}`;
const skipWebServer = Boolean(process.env.E2E_BASE_URL);

export default defineConfig({
  testDir: './e2e',
  timeout: 45_000,
  expect: {
    timeout: 10_000,
  },
  use: {
    baseURL,
    trace: 'on-first-retry',
  },
  webServer: skipWebServer
    ? undefined
    : {
        command: 'npm run build && /usr/local/cargo/bin/cargo run -p mailbox-backend',
        env: {
          ...process.env,
          E2E_TEST_AUTH: 'true',
          E2E_TEST_AUTH_EMAIL: process.env.E2E_TEST_AUTH_EMAIL ?? 'e2e@example.com',
          E2E_TEST_AUTH_NAME: process.env.E2E_TEST_AUTH_NAME ?? 'E2E Tester',
          FRONTEND_DIST: process.env.FRONTEND_DIST ?? 'frontend/dist',
          HOST: '127.0.0.1',
          PORT: String(port),
        },
        reuseExistingServer: true,
        timeout: 120_000,
        url: `${baseURL}/api/health`,
      },
});
