use actix_web::{web,Error,HttpResponse};
use crate::common::{Product};

use crate::repository::{product};

pub async fn get_products() -> Result<HttpResponse,Error>{ 
    let data = product::gets().await;   
    Ok(HttpResponse::Ok().json(data.unwrap()))
}
pub async fn get_productbyid(_id: web::Path<String>) -> Result<HttpResponse,Error>{ 
    let data = product::get(_id.into_inner()).await;   
    Ok(HttpResponse::Ok().json(data.unwrap()))
}
pub async fn update_product(_product: web::Json<Product>) -> Result<HttpResponse,Error>{
    let p:Product = Product{id:_product.id.to_string(),product_type:_product.product_type.to_string(),name:_product.name.to_string()};
    let data = product::update(p).await;
    Ok(HttpResponse::Ok().json(data.unwrap()))
}
pub async fn add_product(_product:web::Json<Product>) -> Result<HttpResponse,Error>{
    let p:Product = Product{id:_product.id.to_string(),product_type:_product.product_type.to_string(),name:_product.name.to_string()};
    let data = product::insert(p).await;
    Ok(HttpResponse::Ok().json(data.unwrap()))
}
pub async fn remove_product(_id :web::Path<String>)->Result<HttpResponse,Error>{
    let data = product::delete(_id.into_inner()).await;  
    Ok(HttpResponse::Ok().json(data.unwrap()))
}

#[cfg(test)]
mod tests{
    use crate::app_config::config_app;
    use actix_web::dev::Service;
    use actix_web::{http::{header,StatusCode},test,App,};

    #[actix_web::test]
    async fn test_add_product(){
        let app = test::init_service(App::new().configure(config_app)).await;
        let payload = r#"{"id":"P001","product_type":"funcy","name":"test"}"#.as_bytes();
        let req = test::TestRequest::post()
        .uri("/products")
        .insert_header((header::CONTENT_TYPE,"application/json"))
        .set_payload(payload)
        .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
}