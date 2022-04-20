use actix_web::{http,web,body,HttpResponse,HttpRequest};
use crate::models::{Product};
use crate::repository::{product};
use crate::errors::MyError;
use crate::auth::{tokens,Claims};

pub async fn get_products(req: HttpRequest) -> Result<HttpResponse,MyError>{ 
    if let Some(hdr) = req.headers().get(http::header::AUTHORIZATION) {
        if let Ok(_s) = hdr.to_str() {
            let decode = tokens::decoder(_s.split_ascii_whitespace().nth(1).unwrap());
            if let Ok(data) = decode{
                let d: Claims = Claims::new(data.sub.clone(),data.name.clone(),data.role,data.iat,data.exp);
                let is_auth = tokens::validation_token(data.name, data.sub).await.unwrap();
                if is_auth{
                    if d.role == "Admin"{
                        let data = product::gets().await?;
                        return Ok(HttpResponse::Ok().json(data));
                    }else{
                        return Ok(HttpResponse::with_body(http::StatusCode::from_u16(401).unwrap(), body::BoxBody::new(String::from("Not Admin"))));
                    }
                }else{
                    return Ok(HttpResponse::with_body(http::StatusCode::from_u16(401).unwrap(), body::BoxBody::new(String::from("Unauthorized"))));
                }
            }
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn add_product(req: HttpRequest,_product:web::Json<Product>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::AUTHORIZATION) {
        if let Ok(_s) = hdr.to_str() {
            let decode = tokens::decoder(_s.split_ascii_whitespace().nth(1).unwrap());
            if let Ok(data) = decode{
                let d: Claims = Claims::new(data.sub.clone(),data.name.clone(),data.role,data.iat,data.exp);
                let is_auth = tokens::validation_token(data.name, data.sub).await.unwrap();
                if is_auth{
                    if d.role == "Admin"{
                        let p:Product = Product{id:_product.id.to_string(),
                        name:_product.name.to_string(),
                        product_type:_product.product_type.to_string(),
                        detail:_product.detail.to_string(),
                        price:_product.price};
                        let data = product::insert(p).await?;
                        return Ok(HttpResponse::Ok().json(data));
                    }else{
                        return Ok(HttpResponse::with_body(http::StatusCode::from_u16(401).unwrap(), body::BoxBody::new(String::from("Not Admin"))));
                    }
                }else{
                    return Ok(HttpResponse::with_body(http::StatusCode::from_u16(401).unwrap(), body::BoxBody::new(String::from("Unauthorized"))));
                }
            }
        }
    }
    Ok(HttpResponse::BadRequest().into())
}