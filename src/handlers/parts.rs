use actix_web::{web,Error,HttpResponse};
use crate::models::{Part};

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

#[cfg(test)]
mod tests{
    use crate::app_config::config_app;
    use actix_web::dev::Service;
    use actix_web::{http::{header,StatusCode},test,App,};

    #[actix_web::test]
    async fn test_add_part(){
        let app = test::init_service(App::new().configure(config_app)).await;
        let payload = r#"{"id":"P001","part_type":"funcy","name":"test"}"#.as_bytes();
        let req = test::TestRequest::post()
        .uri("/parts")
        .insert_header((header::CONTENT_TYPE,"application/json"))
        .set_payload(payload)
        .to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
}