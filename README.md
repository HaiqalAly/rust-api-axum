# Rust API with Axum

A high-performance REST API built with Axum for searching words in an FST (Finite State Transducer) dictionary with fuzzy matching capabilities. This project connects the [FST Experiment](https://github.com/HaiqalAly/rust-fst-exp) to the web, providing a fast and efficient word search service.

## Features

- **Health Monitoring**: Health check endpoint with version information
- **Fast Search**: Lightning-fast word search using FST with memory-mapped files
- **Fuzzy Matching**: Levenshtein automaton-based fuzzy search (edit distance: 1)
- **Smart Ranking**: Top 10 results with intelligent prioritization:
  - 🥇 Exact matches ranked first
  - 🥈 Higher scores ranked second
  - 🥉 Alphabetical ordering as tiebreaker
- **Search History**: Persistent logging of all searches with PostgreSQL
- **Observability**: Request tracing and structured logging with `tracing`
- **Reliability**: Request timeout (10s) and graceful shutdown support
- **Easy Setup**: Docker Compose for quick PostgreSQL deployment

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: Version 1.75 or higher (Rust edition 2024)
- **Docker & Docker Compose**: For running PostgreSQL
- **PostgreSQL**: Version 16 (provided via Docker Compose)
- **FST Dictionary**: Place your dictionary file at `data/dict.fst`

## Quick Start

### 1. Environment Configuration

Copy the example environment file:
```bash
cp .env.example .env
```

### 2. Dictionary Setup

Place your FST dictionary file at:
```
data/dict.fst
```

### 3. Start PostgreSQL

Launch the PostgreSQL database using Docker Compose:
```bash
docker compose up -d
```

### 4. Database Migration

Install SQLx CLI and run migrations:
```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

### 5. Run the Server

Start the API server:
```bash
cargo run
```

**Server is ready!** Access it at: `http://127.0.0.1:8080`

## API Endpoints

### Root Endpoint

**`GET /`**

Returns a simple "Hello, World!" message.

---

### Health Check

**`GET /health`**

Checks the API's health and returns version information.

**Response:**
```json
{
  "status": "Server healthy",
  "version": "0.1.0"
}
```

---

### Word Search

**`GET /search?q=<query>`**

Search for words in the FST dictionary with fuzzy matching (Levenshtein distance: 1). Returns top 10 results prioritized by:
1. Exact matches
2. Higher scores
3. Alphabetical order

**Parameters:**
- `q` (required): The search query string

**Example Request:**
```bash
curl "http://127.0.0.1:8080/search?q=hello"
```

**Example Response:**
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

**Note:** Returns an empty array `[]` if no matches are found.

---

### Search History

**`GET /history`**

Retrieve the last 100 search queries with their results.

**Example Response:**
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