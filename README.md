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

## Checks

```bash
npm run lint
npm run format
PATH=/usr/local/cargo/bin:$PATH cargo fmt --all -- --check
PATH=/usr/local/cargo/bin:$PATH cargo clippy --workspace --all-targets -- -D warnings
```
