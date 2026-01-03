# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Personal website for chrismiller.xyz - a Rust/Axum web application with a PostgreSQL backend and Tailwind CSS frontend.

## Development Setup

**Prerequisites:** Nix with flakes enabled, Docker

```sh
# Enter development environment (or use direnv)
nix develop

# Allow direnv (auto-loads environment on cd)
direnv allow
```

## Local Development (Recommended)

Uses Docker Compose for a local PostgreSQL instance:

```sh
# One command to start everything
just dev

# Or step by step:
just dev-db-up      # Start local PostgreSQL container
just dev-db-migrate # Run migrations (optional - app runs them on startup)
cargo run           # Run the server (DATABASE_URL set by .envrc)

# Stop the database
just dev-db-down

# Reset database to fresh state
just dev-db-reset
```

**Frontend (run in separate terminal):**
```sh
yarn install        # Install dependencies (first time)
yarn watch          # Watch and rebuild CSS
yarn build          # Build CSS for production
```

## Production Database Access

For accessing the k8s-hosted production database (requires kubectl):

```sh
just db-pf          # Port-forward PostgreSQL from k8s (run in separate terminal)
just db             # Get database connection URL
just db-setup       # Initialize diesel
just db-migrate     # Run pending migrations
just db-rollback    # Revert last migration
```

When using port-forwarded k8s PostgreSQL, use `PGSSLMODE=disable`.

## Architecture

**Stack:** Axum (Rust web framework) + Diesel (async PostgreSQL ORM) + markup (compile-time HTML templating) + Tailwind CSS

**Code Structure:**
- `src/main.rs` - Application entry, database pool setup, server initialization
- `src/controllers/` - Route handlers (index, posts, RSS feed)
- `src/models/` - Database models and connection management (`Post` struct, `DatabaseConnection` extractor)
- `src/views/` - HTML templates using the `markup` crate (Layout component, page templates)
- `src/schema.rs` - Diesel-generated database schema (auto-generated, do not edit manually)

**Key Patterns:**
- Database connections are extracted via Axum's `FromRequestParts` trait (`DatabaseConnection` type)
- Views use the `markup::define!` macro for compile-time HTML generation
- All routes use async handlers with connection pool state
- Migrations are embedded and run automatically on startup via `diesel_migrations`

**Routes:**
- `/` - Home page
- `/posts` - Blog post listing
- `/blog/:blog_url` - Individual blog post
- `/rss.xml` - RSS feed
- `/assets/*` - Compiled CSS (from `dist/`)
- `/public/*` - Static files

**CSS Pipeline:** PostCSS processes `styles/main.css` → `dist/main.css` with Tailwind, autoprefixer, and cssnano.
