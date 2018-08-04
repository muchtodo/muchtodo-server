use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::{Request, State, Outcome};
use rocket::request::{self, FromRequest};
use rocket::http::Status;
use std::ops::Deref;

// An alias to the type for a pool of Diesel SQLite connections.
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn {
    pub pool: PooledConnection<ConnectionManager<PgConnection>>
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn{pool: conn}),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

/// Initializes a database pool.
pub fn init_pool(db_url: &str, max_pool: u32) -> PgPool {

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .max_size(max_pool)
        .build(manager)
        .unwrap()
}
