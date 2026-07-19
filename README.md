# Application Scaffold

Two-part application scaffold:

- `frontend/`: React SPA powered by Vite.
- `backend/`: Rust/Axum API and static file server.

## Development

Install JavaScript dependencies:

```bash
npm install
```

Run the frontend dev server:

```bash
npm run dev
```

Build the frontend:

```bash
npm run build
```

Run the end-to-end test suite against a local server:

```bash
set -a
. ./.env.example
set +a
npm run e2e
```

The e2e suite starts the production frontend build and Rust server on `127.0.0.1:8080` by default. It requires a PostgreSQL `DATABASE_URL` and enables the explicit `E2E_TEST_AUTH=true` test-login path for the spawned server only.

Run the backend on `0.0.0.0:8080`:

```bash
set -a
. ./.env.example
set +a
PATH=/usr/local/cargo/bin:$PATH cargo run -p mailbox-backend
```

Run database migrations:

```bash
export DATABASE_URL=postgresql://user:password@host:5432/database
PATH=/usr/local/cargo/bin:$PATH sqlx migrate run --source migrations
```

Copy `.env.example` values into your runtime environment and replace example secrets before running outside local development.

For bare self-hosted deployment without Docker or CI/CD, see [`docs/deployment.md`](docs/deployment.md).

## Checks

```bash
npm run lint
npm run format
npm run e2e
PATH=/usr/local/cargo/bin:$PATH cargo fmt --all -- --check
PATH=/usr/local/cargo/bin:$PATH cargo clippy --workspace --all-targets -- -D warnings
```
