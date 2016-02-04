r2d2-diesel
===========

Provides [r2d2](https://github.com/sfackler/r2d2) support to allow connection
pooling with Diesel for PostgreSQL.

Example
=======

```rust
extern crate r2d2;
extern crate r2d2_diesel;
extern crate diesel;

use std::thread;
use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;

fn main() {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::new("postgres://localhost");
    let pool = r2d2::Pool::new(config, manager).unwrap();

    for i in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
            // Do exciting stuff with the connection!
        });
    }
}
