use diesel::r2d2::ConnectionManager;

use diesel::pg::PgConnection;
use diesel::r2d2;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

static DATABASE_URL: &'static str = env!("DATABASE_URL");

pub fn connect() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    r2d2::Pool::builder().build(manager).expect("Failed to create connection pool.")
}

pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}