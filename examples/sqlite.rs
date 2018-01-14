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

fn main() {
    new_pool();
    new_pool_with_init_operations();
}

fn new_pool() {
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    for _ in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let connection = pool.get();

            assert!(connection.is_ok());
        });
    }
}

const CACHE_SIZE: &'static str = "4000";
fn init_operations(connection: &SqliteConnection) -> Result<(), Error> {
    let query = format!("PRAGMA CACHE_SIZE = {}", CACHE_SIZE);
    connection.execute(&query).map_err(Error::QueryError)?;

    Ok(())
}

fn new_pool_with_init_operations() {
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