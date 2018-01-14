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

use diesel::PgConnection;
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

Or you may need some extra operations after establishing a connection
```rust
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use std::thread;
use diesel::Connection;
use diesel::RunQueryDsl;
use diesel::types::Text;
use diesel::expression::sql_literal::sql;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::{ConnectionManager, Error};

const CACHE_SIZE: &'static str = "4000";
fn init_operations(connection: &SqliteConnection) -> Result<(), Error> {
    let query = format!("PRAGMA CACHE_SIZE = {}", CACHE_SIZE);
    connection.execute(&query).map_err(Error::QueryError)?;

    Ok(())
}

fn main() {
    let manager =
        ConnectionManager::<SqliteConnection>::new_with_init_operations("db.sqlite",
                                                                        Box::new(init_operations));
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    for _ in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let query = "PRAGMA CACHE_SIZE";
            let connection = pool.get().expect("Failed to get pooled connection");
            let cache_size = sql::<Text>(&query).load::<String>(&*connection)
                .expect("Failed to get cache size").pop();

            assert_eq!(Some(CACHE_SIZE.to_owned()), cache_size);
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

