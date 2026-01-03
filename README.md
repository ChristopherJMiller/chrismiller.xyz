# chrismiller.xyz

Personal website built with Rust/Axum, PostgreSQL, and Tailwind CSS.

## Quick Start (Local Development)

**Prerequisites:** [Nix](https://nixos.org/download.html) with flakes enabled, Docker

```sh
# Enter development environment
nix develop
# Or if using direnv:
direnv allow

# Install frontend dependencies
yarn install

# Start local PostgreSQL + run the app
just dev

# In a separate terminal, watch for CSS changes
yarn watch
```

The app will be available at http://localhost:3000

## Development Commands

### Local Development (Docker Compose)

| Command | Description |
|---------|-------------|
| `just dev` | Start local PostgreSQL and run the app |
| `just dev-db-up` | Start local PostgreSQL container |
| `just dev-db-down` | Stop local PostgreSQL container |
| `just dev-db-reset` | Reset database (removes all data) |
| `just dev-db-setup` | Initialize diesel |
| `just dev-db-migrate` | Run pending migrations |
| `just dev-db-rollback` | Revert last migration |

### Production Database (requires kubectl)

| Command | Description |
|---------|-------------|
| `just db-pf` | Port-forward PostgreSQL from k8s |
| `just db` | Get database connection URL |
| `just db-setup` | Initialize diesel |
| `just db-migrate` | Run pending migrations |
| `just db-rollback` | Revert last migration |

### Frontend

| Command | Description |
|---------|-------------|
| `yarn install` | Install dependencies |
| `yarn watch` | Watch and rebuild CSS |
| `yarn build` | Build CSS for production |

## Environment

The development environment is managed via Nix flake. It provides:
- Rust toolchain (from `rust-toolchain.toml`)
- Node.js 20 + Yarn
- PostgreSQL client
- Diesel CLI
- Docker + Docker Compose
- Just (task runner)

The `.envrc` file automatically sets `DATABASE_URL` for local development when using direnv.
