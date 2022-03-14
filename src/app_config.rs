use actix_web::web;
use crate::handlers::{parts,products};

pub fn config_app(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/products")
        .service(
            web::resource("")
            .route(web::get().to(products::get_products))
            .route(web::put().to(products::update_product))
            .route(web::post().to(products::add_product)),
        )
        .service(
            web::scope("/{product_id}")
            .service(
                web::resource("")
                .route(web::get().to(products::get_productbyid))               
                .route(web::delete().to(products::remove_product)),
            )           
        ),       
    ).service(
        web::scope("/parts")
        .service(
            web::resource("")
            .route(web::get().to(parts::get_part))
            .route(web::put().to(parts::update_part))
            .route(web::post().to(parts::add_part)),
        )
        .service(
            web::resource("/{part_id}")
            .route(web::get().to(parts::get_partbyid))
            .route(web::delete().to(parts::remove_part)),
        ),       
    );
}