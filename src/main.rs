mod models;
mod schema;
mod basic_auth;
mod repositories;

use rocket::{get, launch, routes, catch, delete, put, post, catchers};
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket_sync_db_pools::database;
use models::{NewProduct, Product};
use basic_auth::BasicAuthStruct;
use repositories::ProductRepository;

// ================================= sqlite ==============================
#[macro_use] extern crate diesel;
#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

// ================================= 路由CRUD ==============================

#[get("/")]
async fn get_products(conn: DbConn) -> Value {
    conn.run(
        |con| {
            let products = ProductRepository::find_all(con).expect("Error products list");
            json!(products)
        }
    ).await
}

#[get("/<id>")]
async fn view_product(id: i32, conn: DbConn) -> Value {
    conn.run(
        move |con| {
            let products = ProductRepository::find(con, id).expect("Error Get");
            json!(products)
        }
    ).await
}

#[post("/", format="json", data="<new_product>")]
async fn create_product(_auth: BasicAuthStruct, conn: DbConn, new_product: Json<NewProduct>) -> Value {
    // println!("{} {}", _auth.username, _auth.password);
    conn.run({
        |con| {
            let result = ProductRepository::create(con, new_product.into_inner()).expect("Error create product");
            json!(result)
        }
    }).await
}

#[put("/<id>", format="json", data="<product>")]
async fn put_product(id: i32, _auth: BasicAuthStruct, conn: DbConn, product: Json<Product>) -> Value {
    conn.run({
      move |con| {
        let reuslt = ProductRepository::save(con, product.into_inner()).expect("Error Update");
        json!(reuslt)
      }  
    }).await
}

#[delete("/<id>")]
async fn delete_product(id: i32, _auth: BasicAuthStruct, coon: DbConn) -> Value {
    coon.run(
        move |con| {
            let result = ProductRepository::delete(con, id).expect("Error to Del");
            json!(result)
        }
    ).await
}

#[catch(404)]
fn not_found_url() -> Value {
    json!("404 not found!")
}

// ================================= Main ==============================

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/product", routes![get_products, view_product, create_product, put_product, delete_product])
        .register("/", catchers![not_found_url])
        .attach(DbConn::fairing())
}