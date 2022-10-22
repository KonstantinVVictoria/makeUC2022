#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
use rocket_sync_db_pools::diesel as rs_diesel;

mod crud;
mod model;
mod schema;

#[database("plants_db")]
pub struct DbConn(rs_diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
