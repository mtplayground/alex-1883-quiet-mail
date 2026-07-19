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
PATH=/usr/local/cargo/bin:$PATH cargo run --workspace
```

## Checks

```bash
npm run lint
npm run format
PATH=/usr/local/cargo/bin:$PATH cargo fmt --all -- --check
PATH=/usr/local/cargo/bin:$PATH cargo clippy --workspace --all-targets -- -D warnings
```
