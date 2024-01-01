use base64::Engine;
use rocket::{get, launch, routes, catch, delete, put, post, catchers};
use rocket::serde::json::{Value, serde_json::json};
use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;
use rocket_sync_db_pools::database;

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

// ================================= 路由 ==============================

#[get("/")]
fn get_products() -> Value {
    json!("list")
}

#[get("/<id>")]
fn view_product(id: i32) -> Value {
    json!("get")
}

#[post("/")]
fn create_product(_auth: BasicAuthStruct) -> Value {
    // println!("{} {}", _auth.username, _auth.password);
    json!("create")
}

#[put("/<id>")]
fn put_product(id: i32, _auth: BasicAuthStruct) -> Value {
    json!("put")
}

#[delete("/<id>")]
fn delete_product(id: i32, _auth: BasicAuthStruct) -> Value {
    json!("delete")
}

#[catch(404)]
fn not_found_url() -> Value {
    json!("404 not found!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/product", routes![get_products, view_product, create_product, put_product, delete_product])
        .register("/", catchers![not_found_url])
        .attach(DbConn::fairing())
}