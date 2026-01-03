# axum-tutorial

A sample TODO app written in Rust with SQLite

- Async runtime: [Tokio](https://tokio.rs)
- Web framework: [Axum](https://github.com/tokio-rs/axum)
- Database: [SQLite](https://www.sqlite.org)
- OpenAPI annotation provider: [utoipa](https://github.com/juhaku/utoipa)
- OpenAPI client page: [Scalar](https://scalar.com)
- ORM: [SeaORM](https://www.sea-ql.org/SeaORM)

## Setup

```console
sqlite3 data/app.db < sql/create-tables.sql
cargo run
```

## Run


```console
LISTEN_ADDRESS=0.0.0.0:8080 DATABASE_URL=sqlite://data/app.db target/debug/axum-tutorial
```

> [http://localhost:8080/scalar](http://localhost:8080/scalar)
