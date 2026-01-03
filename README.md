# axum-tutorial

A sample TODO app written in Rust with SQLite

- Async runtime: Tokio
- Web framework: Axum
- OpenAPI annotation provider: utoipa
- Database: SQLite
- ORM: SeaORM

## Setup

```console
sqlite3 data/app.db < sql/create-tables.sql
cargo build --release
```

## Run

```console
LISTEN_ADDRESS=0.0.0.0:8080 DATABASE_URL=sqlite://data/app.db target/release/axum-tutorial
``` 

> [http://localhost:8080/swagger](http://localhost:8080/swagger)
