extern crate diesel;
extern crate r2d2;

use diesel::{Connection, ConnectionError};
use r2d2::ManageConnection;
use std::convert::Into;

pub struct ConnectionManager {
    database_url: String,
}

impl ConnectionManager {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        ConnectionManager {
            database_url: database_url.into(),
        }
    }
}

pub enum Error {
    ConnectionError(ConnectionError),
    QueryError(diesel::result::Error),
}

impl ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Connection, Error> {
        Connection::establish(&self.database_url)
            .map_err(Error::ConnectionError)
    }

    fn is_valid(&self, conn: &mut Connection) -> Result<(), Error> {
        conn.execute("SELECT 1").map(|_| ()).map_err(Error::QueryError)
    }

    fn has_broken(&self, _conn: &mut Connection) -> bool {
        false
    }
}
