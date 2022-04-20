use actix_web::{http,web,body,HttpResponse,HttpRequest};
use crate::models::{User};
use crate::repository::{user};
use crate::errors::MyError;
use crate::auth::{tokens};

pub async fn get_users(req: HttpRequest) -> Result<HttpResponse,MyError>{ 
    if let Some(hdr) = req.headers().get(http::header::AUTHORIZATION) {
        if let Ok(_s) = hdr.to_str() {
            let decode = tokens::decoder(_s.split_ascii_whitespace().nth(1).unwrap());
            if let Ok(data) = decode{
                println!("{:#?}",data);
                let data = user::gets().await?;
                return Ok(HttpResponse::Ok().json(data));
            }else{
                return Ok(HttpResponse::with_body(http::StatusCode::from_u16(401).unwrap(), body::BoxBody::new(String::from("No Authorized"))));
            }           
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn get_user(req: HttpRequest,_id: web::Path<String>) -> Result<HttpResponse,MyError>{ 
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = user::get(_id.into_inner()).await?;
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn update_user(req: HttpRequest,_user: web::Json<User>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let p:User = User{id:_user.id,
                name:_user.name.to_string(),
                password:_user.password.to_string(),
                email:_user.email.to_string()};
            let data = user::update(p).await?;
            return Ok(HttpResponse::Ok().json(data));
        } 
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn add_user(req: HttpRequest,_user:web::Json<User>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let p:User = User{id:_user.id,
                name:_user.name.to_string(),
                password:_user.password.to_string(),
                email:_user.email.to_string()};
            let data = user::insert(p).await?;
            return Ok(HttpResponse::Ok().json(data));   
        }
    } 
    Ok(HttpResponse::BadRequest().into())
}
pub async fn remove_user(req: HttpRequest,_id :web::Path<i32>)->Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = user::delete(_id.into_inner()).await?;
            return Ok(HttpResponse::Ok().json(data));  
        }
    }
    Ok(HttpResponse::BadRequest().into())
}