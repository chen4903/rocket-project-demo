#[macro_use] extern crate diesel;

mod models;
mod database;
mod auth;
mod repositories;
mod controller;

use rocket::{launch, routes, catchers};
use controller::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/product", routes![get_products, view_product, create_product, put_product, delete_product])
        .register("/", catchers![not_found_url])
        .attach(DbConn::fairing())
}