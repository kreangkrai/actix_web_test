use actix_web::{web,Error,HttpResponse};
use crate::common::{Part};

use crate::repository::{part};

pub async fn get_part() -> Result<HttpResponse,Error>{ 
    let data = part::gets().await;   
    Ok(HttpResponse::Ok().json(data.unwrap()))
}
pub async fn get_partbyid(_id: web::Path<String>) -> Result<HttpResponse,Error>{ 
    let data = part::get(_id.into_inner()).await;   
    Ok(HttpResponse::Ok().json(data.unwrap()))
}
pub async fn update_part(_part: web::Json<Part>) -> Result<HttpResponse,Error>{
    let p:Part = Part{id:_part.id.to_string(),part_type:_part.part_type.to_string(),name:_part.name.to_string()};
    let data = part::update(p).await;
    Ok(HttpResponse::Ok().json(data.unwrap()))
}
pub async fn add_part(_part:web::Json<Part>) -> Result<HttpResponse,Error>{
    let p:Part = Part{id:_part.id.to_string(),part_type:_part.part_type.to_string(),name:_part.name.to_string()};
    let data = part::insert(p).await;
    Ok(HttpResponse::Ok().json(data.unwrap()))
}
pub async fn remove_part(_id :web::Path<String>)->Result<HttpResponse,Error>{
    let data = part::delete(_id.into_inner()).await;  
    Ok(HttpResponse::Ok().json(data.unwrap()))
}