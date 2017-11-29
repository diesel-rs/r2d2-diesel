r2d2-diesel
===========

Provides [r2d2](https://github.com/sfackler/r2d2) support to allow connection pooling with Diesel.

Examples
========

The examples creates a connection pool with default settings for a PostgreSQL or
SQLite database running on localhost, then creates a bunch of threads and
acquires a connection from the pool for each thread.

Executable versions are in [examples/](examples/) which you can run with
`cargo run --example postgres --features "diesel/postgres"` or
`cargo run --example sqlite --features "diesel/sqlite"`.


```rust
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use std::thread;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

fn main() {
    let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    for _ in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let connection = pool.get();

            assert!(connection.is_ok());
        });
    }
}
```

Using diesel master branch
============================

If you want to use diesel master's branch with r2d2-diesel you have to add the
following section in your Cargo.toml file. If you're using a workspace, this
needs to be in the Cargo.toml at the root of the workspace.

```toml
[patch.crates-io]
diesel = { git = "https://github.com/diesel-rs/diesel.git" }
diesel_infer_schema = { git = "https://github.com/diesel-rs/diesel.git" }
diesel_codegen = { git = "https://github.com/diesel-rs/diesel.git" }
```

