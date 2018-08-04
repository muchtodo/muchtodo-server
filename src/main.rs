#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

mod db;
mod schema;
mod models;
mod guards;
mod api;

use rocket::fairing::AdHoc;
use db::*;


#[get("/")]
fn index() -> &'static str {
    "Congratulations! You've set up your Muchtodo server successfully."
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_attach(|rocket| {
            // create managed pool from db url config var
            let config = rocket.config().clone();
            let db_url = config.get_str("database_url").expect("No `database_url` specified in Rocket.toml");
            let max_pool = config.get_int("database_max_pool").unwrap_or(8);
            let pool = init_pool(db_url, max_pool as u32);
            Ok(rocket.manage(pool))
        }))
        .mount("/", routes![index])
        .mount("/api/1/", 
            routes![
                api::get_tasks, 
                api::get_tasks_active,
                api::get_tasks_toplevel,
                api::get_tasks_belongingto,
                api::get_tasks_completed,
            ])
        .launch();
}
