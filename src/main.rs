#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

use rocket::fairing::AdHoc;
use rocket_contrib::{Json, Value, Template};
use rocket::http::{Header, Status};
use diesel::prelude::*;
use diesel::result::Error;
use db::*;

mod db;
mod schema;
mod models;
mod guards;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tasks")]
fn get_tasks(conn: DbConn, auth: guards::AccessTokenAuth) -> Result<Json<Vec<models::Task>>, Error> {
    use ::schema::tasks::dsl::*;

    match models::Task::belonging_to(&auth.user).load::<models::Task>(&*conn) {
        Ok(t) => Ok(Json(t)),
        Err(e) => Err(e)
    }    
    
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_attach(|rocket| {
            // create managed pool from db url config var
            let config = rocket.config().clone();
            let db_url = config.get_str("database_url").expect("No `database_url` specified in Rocket.toml");
            let pool = init_pool(db_url);
            Ok(rocket.manage(pool))
        }))
        .mount("/", routes![index])
        .mount("/api/1/", routes![get_tasks])
        .launch();
}
