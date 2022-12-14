use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    dotenv().ok();

    // let database_url = "postgresql://db:5432/glaceon_db?user=glaceon&password=glaceon1234";
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}
