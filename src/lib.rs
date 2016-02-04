extern crate diesel;
extern crate r2d2;

use diesel::{Connection, ConnectionError};
use r2d2::ManageConnection;
use std::convert::Into;
use std::marker::PhantomData;

pub struct ConnectionManager<T> {
    database_url: String,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send + 'static> Sync for ConnectionManager<T> {
}

impl<T> ConnectionManager<T> {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        ConnectionManager {
            database_url: database_url.into(),
            _marker: PhantomData,
        }
    }
}

pub enum Error {
    ConnectionError(ConnectionError),
    QueryError(diesel::result::Error),
}

impl<T> ManageConnection for ConnectionManager<T> where
    T: Connection + Send + 'static,
{
    type Connection = T;
    type Error = Error;

    fn connect(&self) -> Result<T, Error> {
        T::establish(&self.database_url)
            .map_err(Error::ConnectionError)
    }

    fn is_valid(&self, conn: &mut T) -> Result<(), Error> {
        conn.execute("SELECT 1").map(|_| ()).map_err(Error::QueryError)
    }

    fn has_broken(&self, _conn: &mut T) -> bool {
        false
    }
}
