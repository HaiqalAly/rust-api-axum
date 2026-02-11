# Rust API with Axum

A simple REST API built with Axum that searches words in an FST (Finite State Transducer) dictionary.

## Features

- Health check endpoint
- Word search using FST for fast lookups
- PostgreSQL connection ready
- Request tracing and logging
- Graceful shutdown

## Prerequisites

- Rust 1.75+
- Docker
- FST dictionary file at `data/dict.fst`

## Setup

1. Copy `.env.example` to `.env` and set your `DATABASE_URL`
2. Place your FST dictionary file in `data/dict.fst`
3. Run the server:

```bash
docker compose up -d

cargo run
```

Server starts on `http://127.0.0.1:8080`

## Endpoints

**GET /** - Hello world

**GET /health** - Health check with version info

**GET /search?q=word** - Search for a word in the FST dictionary

Example:
```bash
curl "http://127.0.0.1:8080/search?q=hello"
```

Response:
```json
[
  {
    "found": "hello",
    "score": "12345",
    "exist": true
  }
]
```

Returns empty array if word not found.