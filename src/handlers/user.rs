use actix_web::{http,web,HttpResponse,HttpRequest};
use crate::models::{User};
use crate::repository::{user};
use crate::errors::MyError;

pub async fn get_users(req: HttpRequest) -> Result<HttpResponse,MyError>{ 
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = user::gets().await?;
            return Ok(HttpResponse::Ok().json(data));
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