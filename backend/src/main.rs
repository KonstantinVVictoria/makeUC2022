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
        // INDEX
        .mount("/", routes![index])
        // /api/plant
        .mount(
            "/api/plant",
            routes![
                crud::create_plant,
                crud::read_plant,
                crud::read_all_plants,
                crud::update_plant,
                crud::delete_plant
            ],
        )
        // /api/users
        .mount(
            "/api/users",
            routes![
                crud::create_user,
                crud::read_user,
                crud::update_user,
                crud::delete_user
            ],
        )
        .attach(DbConn::fairing())
}
