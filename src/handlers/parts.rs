use actix_web::{web,HttpResponse,HttpRequest,http};
use crate::models::{Part};
use crate::errors::MyError;
use crate::repository::{part};

pub async fn get_part(req: HttpRequest) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = part::gets().await?;   
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn get_partbyid(req: HttpRequest,_id: web::Path<String>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = part::get(_id.into_inner()).await?;   
            return Ok(HttpResponse::Ok().json(data));   
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn update_part(req: HttpRequest,_part: web::Json<Part>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let p:Part = Part{id:_part.id.to_string(),part_type:_part.part_type.to_string(),name:_part.name.to_string()};
            let data = part::update(p).await?;
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn add_part(req: HttpRequest,_part:web::Json<Part>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let p:Part = Part{id:_part.id.to_string(),part_type:_part.part_type.to_string(),name:_part.name.to_string()};
            let data = part::insert(p).await?;
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn remove_part(req: HttpRequest,_id :web::Path<String>)->Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = part::delete(_id.into_inner()).await?;  
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}