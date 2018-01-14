extern crate diesel;
extern crate r2d2;

use diesel::{Connection, ConnectionError};
use r2d2::ManageConnection;
use std::convert::Into;
use std::fmt;
use std::marker::PhantomData;

pub struct ConnectionManager<T> {
    database_url: String,
    init_operations: Option<BoxedInitOperations<T>>,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send + 'static> Sync for ConnectionManager<T> {
}

pub trait InitOperations<T>: Fn(&T) -> Result<(), Error> + Sync + 'static + Send {}
impl<T, F> InitOperations<T> for F where F: Fn(&T) -> Result<(), Error> + Sync + 'static + Send {}
pub type BoxedInitOperations<T> = Box<InitOperations<T, Output=Result<(), Error>>>;

impl<T> ConnectionManager<T> {
    pub fn new<S: Into<String>>(database_url: S) -> Self
    {
        ConnectionManager {
            database_url: database_url.into(),
            init_operations: None,
            _marker: PhantomData,
        }
    }

    pub fn new_with_init_operations<S: Into<String>>(
        database_url: S,
        init_operations: BoxedInitOperations<T>
    ) -> Self
    {
        ConnectionManager {
            database_url: database_url.into(),
            init_operations: Some(init_operations),
            _marker: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ConnectionError(ConnectionError),
    QueryError(diesel::result::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ConnectionError(ref e) => e.fmt(f),
            Error::QueryError(ref e) => e.fmt(f),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConnectionError(ref e) => e.description(),
            Error::QueryError(ref e) => e.description(),
        }
    }
}

impl<T> ManageConnection for ConnectionManager<T> where
    T: Connection + Send + 'static,
{
    type Connection = T;
    type Error = Error;

    fn connect(&self) -> Result<T, Error> {
        let conn = T::establish(&self.database_url).map_err(Error::ConnectionError)?;
        if let Some(init_operations) = self.init_operations.as_ref() {
            init_operations(&conn)?;
        }

        Ok(conn)
    }

    fn is_valid(&self, conn: &mut T) -> Result<(), Error> {
        conn.execute("SELECT 1").map(|_| ()).map_err(Error::QueryError)
    }

    fn has_broken(&self, _conn: &mut T) -> bool {
        false
    }
}
