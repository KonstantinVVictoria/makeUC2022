use rocket_sync_db_pools::rocket::launch;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod crud;
mod model;
mod schema;

#[database("plants_db")]
pub struct DbConn(diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api/create", routes![crud::create_plant])
        .mount("/api/read", routes![crud::read_plant])
        .mount("/api/update", routes![crud::update_plant])
        .mount("/api/delete", routes![crud::delete_plant])
        .attach(DbConn::fairing())
}
