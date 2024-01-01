mod models;
mod schema;

use base64::Engine;
use rocket::{get, launch, routes, catch, delete, put, post, catchers};
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;
use rocket_sync_db_pools::database;
use schema::products;
use models::{NewProduct, Product};
use diesel::{ExpressionMethods, RunQueryDsl, query_dsl::methods::{FindDsl, LimitDsl}};

// ========================= 从请求体中验证请求者的身份，使用base64(不安全，仅演示)==============================

pub struct BasicAuthStruct {
    pub username: String,
    pub password: String,
}

impl BasicAuthStruct {
    // 加密的"Basic username:password" 
    fn from_header(header: &str) -> Option<BasicAuthStruct> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        if split_vec[0] != "Basic" {
            return None;
        }
        // base64 
        Self::from_base64(split_vec[1])
    }

    // 字符串 => 结构体
    fn from_base64(base64_string: &str) -> Option<BasicAuthStruct>{
        let decoder = base64::engine::general_purpose::STANDARD.decode(base64_string).ok()?;
        let decode_str = String::from_utf8(decoder).ok()?; // username:password" 
        let split_str = decode_str.split(":").collect::<Vec<_>>();
        if split_str.len() != 2 {
            return None;
        }
        let (username, password) = (split_str[0].to_string(), split_str[1].to_string());
        Some(BasicAuthStruct {
            username,
            password
        })
    }

}

#[rocket::async_trait]
impl <'a>FromRequest<'a> for BasicAuthStruct {
    type Error = ();
    
    async fn from_request(request: &'a rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_header(header_auth) {
                return Outcome::Success(auth)
            }
        }
        Outcome::Error((Status::Unauthorized, ())) // 401: Unauthorized
    }
}

// ================================= sqlite ==============================
#[macro_use] extern crate diesel;
#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

// ================================= 路由CRUD ==============================

#[get("/")]
async fn get_products(conn: DbConn) -> Value {
    conn.run(
        |con| {
            let products = products::table.limit(100).load::<Product>(con).expect("Error products list");
            json!(products)
        }
    ).await
}

#[get("/<id>")]
async fn view_product(id: i32, conn: DbConn) -> Value {
    conn.run(
        move |con| {
            let products = products::table.find(id).get_result::<Product>(con).expect("Error Get");
            json!(products)
        }
    ).await
}

#[post("/", format="json", data="<new_product>")]
async fn create_product(_auth: BasicAuthStruct, conn: DbConn, new_product: Json<NewProduct>) -> Value {
    // println!("{} {}", _auth.username, _auth.password);
    conn.run({
        |con| {
            let result = diesel::insert_into(products::table)
                            .values(new_product.into_inner())
                            .execute(con)
                            .expect("Error create product");
            json!(result)
        }
    }).await
}

#[put("/<id>", format="json", data="<product>")]
async fn put_product(id: i32, _auth: BasicAuthStruct, conn: DbConn, product: Json<Product>) -> Value {
    conn.run({
      move |con| {
        let reuslt = diesel::update(products::table.find(id))
                                .set((
                                    products::name.eq(product.name.to_owned()),
                                    products::description.eq(product.description.to_owned())
                                ))
                                .execute(con)
                                .expect("Error Update");
        json!(reuslt)
      }  
    }).await
}

#[delete("/<id>")]
async fn delete_product(id: i32, _auth: BasicAuthStruct, coon: DbConn) -> Value {
    coon.run(
        move |con| {
            let result = diesel::delete(products::table.find(id))
                .execute(con)
                .expect("Error to Del");
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