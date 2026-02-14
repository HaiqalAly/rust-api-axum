# Rust API with Axum

A REST API built with Axum that searches words in an FST (Finite State Transducer) dictionary with fuzzy matching. This was made to connect my [FST Experiment](https://github.com/HaiqalAly/rust-fst-exp) to the web.

## Features

- Health check endpoint with version info
- Fast word search using FST with memory-mapped files
- Fuzzy search using Levenshtein automaton (distance 1)
- Top 10 results using BinaryHeap with prioritization:
  - Exact matches ranked first
  - Higher scores ranked second
  - Alphabetical ordering as tiebreaker
- Search history logging with PostgreSQL
- Request tracing and structured logging
- Request timeout (10s) and graceful shutdown
- Docker Compose for PostgreSQL setup

## Prerequisites

- Rust 1.75+ (edition 2024)
- Docker and Docker Compose
- PostgreSQL 16 (via Docker)
- FST dictionary file at `data/dict.fst`

## Setup

1. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```

2. Place your FST dictionary file in `data/dict.fst`

3. Start PostgreSQL using Docker Compose:
   ```bash
   docker compose up -d
   ```

4. Run database migrations:
   ```bash
   cargo install sqlx-cli --no-default-features --features postgres
   sqlx migrate run
   ```

5. Build and run the server:
   ```bash
   cargo run
   ```

Server starts on `http://127.0.0.1:8080`

## Endpoints

### `GET /`
Returns a simple "Hello, World!" message.

### `GET /health`
Health check endpoint with version information.

**Response:**
```json
{
  "status": "Server healthy",
  "version": "0.1.0"
}
```

### `GET /search?q=<query>`
Search for a word in the FST dictionary with fuzzy matching (Levenshtein distance 1).
Returns top 10 results prioritized by exact match, score, and alphabetically.

**Example:**
```bash
curl "http://127.0.0.1:8080/search?q=hello"
```

**Response:**
```json
[
  {
    "found": "hello",
    "score": "12345",
    "exist": true
  },
  {
    "found": "hallo",
    "score": "11000",
    "exist": true
  }
]
```

Returns empty array `[]` if no matches found.

### `GET /history`
Retrieve the last 100 search queries with their results.

**Response:**
```json
[
  {
    "id": 1,
    "query": "hello",
    "found": true,
    "searched_at": "2026-02-14T10:30:00"
  }
]
```

## Technologies

- **Axum** - Web framework
- **SQLx** - Async PostgreSQL driver with compile-time query verification
- **FST** - Finite State Transducer for efficient dictionary storage
- **Tokio** - Async runtime
- **Tower HTTP** - Middleware for tracing and timeouts
- **Tracing** - Structured logging and diagnostics

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string (default: `postgresql://postgres:postgres@localhost:5432/rustdb`)

## Database Schema

```sql
CREATE TABLE search_history (
    id SERIAL PRIMARY KEY,
    query TEXT NOT NULL,
    found BOOLEAN NOT NULL,
    searched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Development

The project uses:
- Rust edition 2024
- Compile-time query verification with SQLx
- Memory-mapped FST files for performance
- Async background logging to avoid blocking requests
- Structured logging with tracing-subscriber

## License

See [LICENSE](LICENSE) file for details.