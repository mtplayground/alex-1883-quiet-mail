# Bare Self-Hosted Deployment

This app deploys as a Rust/Axum server that serves the Vite build from `frontend/dist`.
There is no Dockerfile and no CI/CD requirement.

## Server Prerequisites

- Linux host with systemd or another process supervisor
- Node.js 20 or newer
- Rust toolchain available on `PATH`
- PostgreSQL 16 database reachable through `DATABASE_URL`
- Network access to the Ideavibes auth JWKS endpoint and email service
- TLS termination in front of the app, for example Caddy, nginx, or a managed load balancer

## Required Environment

Set these variables in the service environment:

```bash
HOST=0.0.0.0
PORT=8080
FRONTEND_DIST=/opt/alex-1883-quiet-mail/frontend/dist
SELF_URL=https://mail.example.com

DATABASE_URL=postgresql://app_user:app_password@postgres.example.com:5432/app_database?sslmode=require
DATABASE_MAX_CONNECTIONS=5

MCTAI_AUTH_URL=https://auth.mctai.app
MCTAI_AUTH_APP_TOKEN=app_alex-1883-quiet-mail-85679b
MCTAI_AUTH_JWKS_URL=https://auth.mctai.app/.well-known/jwks.json

MCTAI_EMAIL_URL=https://email.mctai.app/send
MCTAI_EMAIL_APP_TOKEN=replace-with-provisioned-token
```

Do not set `E2E_TEST_AUTH` in production. It is only for the Playwright test server.

## Build

From the repository checkout:

```bash
npm ci
npm run build
cargo build -p mailbox-backend --release
```

## Database

The server runs embedded migrations on startup. To run them manually before the first boot:

```bash
export DATABASE_URL=postgresql://app_user:app_password@postgres.example.com:5432/app_database?sslmode=require
sqlx migrate run --source migrations
```

## Start Command

```bash
./target/release/mailbox-backend
```

The process listens on `HOST:PORT` and serves the built frontend from `FRONTEND_DIST`.

## systemd Unit

```ini
[Unit]
Description=Mailbox application
After=network-online.target
Wants=network-online.target

[Service]
WorkingDirectory=/opt/alex-1883-quiet-mail
EnvironmentFile=/etc/alex-1883-quiet-mail.env
ExecStart=/opt/alex-1883-quiet-mail/target/release/mailbox-backend
Restart=always
RestartSec=5
User=alex-1883-quiet-mail
Group=alex-1883-quiet-mail

[Install]
WantedBy=multi-user.target
```

Reload and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now alex-1883-quiet-mail
sudo systemctl status alex-1883-quiet-mail
```

## Smoke Checks

```bash
curl -fsS http://127.0.0.1:8080/api/health
curl -fsS http://127.0.0.1:8080/api/health/db
```

Run the browser e2e suite against a staging environment only if that environment explicitly enables
`E2E_TEST_AUTH=true`:

```bash
E2E_BASE_URL=https://staging-mail.example.com npm run e2e
```

For local e2e runs, leave `E2E_BASE_URL` unset and provide a PostgreSQL `DATABASE_URL`; Playwright will start the app with `E2E_TEST_AUTH=true`.
