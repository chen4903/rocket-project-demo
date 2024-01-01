use rocket::{request::{FromRequest, Outcome}, http::Status};
use base64::Engine;

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