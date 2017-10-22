use std::error::Error;
use std::ops::Deref;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::{Config, Pool, PooledConnection};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

type DbManager = ConnectionManager<PgConnection>;
pub type DbConnectionPool = Pool<DbManager>;

use super::AppState;

pub fn pool(db_string: &str) -> Result<DbConnectionPool, Box<Error>> {
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(db_string);
    let pool = Pool::new(config, manager)?;

    Ok(pool)
}

pub struct DbConnection(pub PooledConnection<DbManager>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConnection, ()> {
        let pool = &request.guard::<State<AppState>>()?.db_pool;

        match pool.get() {
            Ok(connection) => Outcome::Success(DbConnection(connection)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}