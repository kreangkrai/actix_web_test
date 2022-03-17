use actix_web::{http,web,HttpResponse,HttpRequest};
use crate::models::{Product};
use crate::repository::{product};
use crate::errors::MyError;

pub async fn get_products(req: HttpRequest) -> Result<HttpResponse,MyError>{ 
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = product::gets().await?;
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn get_productbyid(req: HttpRequest,_id: web::Path<String>) -> Result<HttpResponse,MyError>{ 
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = product::get(_id.into_inner()).await?;
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn update_product(req: HttpRequest,_product: web::Json<Product>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let p:Product = Product{id:_product.id.to_string(),product_type:_product.product_type.to_string(),name:_product.name.to_string()};
            let data = product::update(p).await?;
            return Ok(HttpResponse::Ok().json(data));
        } 
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn add_product(req: HttpRequest,_product:web::Json<Product>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let p:Product = Product{id:_product.id.to_string(),product_type:_product.product_type.to_string(),name:_product.name.to_string()};
            let data = product::insert(p).await?;
            return Ok(HttpResponse::Ok().json(data));   
        }
    } 
    Ok(HttpResponse::BadRequest().into())
}
pub async fn remove_product(req: HttpRequest,_id :web::Path<String>)->Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = product::delete(_id.into_inner()).await?;
            return Ok(HttpResponse::Ok().json(data));  
        }
    }
    Ok(HttpResponse::BadRequest().into())
}