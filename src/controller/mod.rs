use rocket::{get, catch, delete, put, post};
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket_sync_db_pools::database;
use rocket::response::status;
use rocket::http::Status;
use crate::repositories::*;
use crate::auth::*;
use crate::models::*;

#[database("sqlite_path")]
pub struct DbConn(diesel::SqliteConnection);

#[get("/")]
pub async fn get_products(conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(
        |con| {
            ProductRepository::find_all(con)
                .map(|product| json!(product))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }
    ).await
}

#[get("/<id>")]
pub async fn view_product(id: i32, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(
        move |con| {
            ProductRepository::find(con, id)
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }).await
}

#[post("/", format="json", data="<new_product>")]
pub async fn create_product(
    _auth: BasicAuthStruct,
    conn: DbConn,
    new_product: Json<NewProduct>
) -> Result<Value, status::Custom<Value>> {
    conn.run({
        |con| {
            ProductRepository::create(con, new_product.into_inner())
                .map(|product| json!(product))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }
    }).await
}

#[put("/<id>", format="json", data="<product>")]
pub async fn put_product(
    id: i32,
    _auth: BasicAuthStruct,
    conn: DbConn,
    product: Json<Product>
) -> Result<Value, status::Custom<Value>> {

    conn.run({
      move |con| {
        ProductRepository::save(con, product.into_inner())
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
      }  
    }).await
}

#[delete("/<id>")]
pub async fn delete_product(id: i32, _auth: BasicAuthStruct, coon: DbConn) -> Result<Value, status::Custom<Value>> {
    coon.run(
        move |con| {
            ProductRepository::delete(con, id)
                .map(|product| json!(product))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }
    ).await
}

#[catch(404)]
pub fn not_found_url() -> Value {
    json!("404 not found!")
}