use actix_web::web;
use crate::handlers::{user,product};

pub fn config_app(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/users")
        .service(
            web::resource("")
            .route(web::get().to(user::get_users))
            .route(web::put().to(user::update_user))
            .route(web::post().to(user::add_user)),
        )
        .service(
            web::scope("/{user_id}")
            .service(
                web::resource("")
                .route(web::get().to(user::get_user))               
                .route(web::delete().to(user::remove_user)),
            )
        ),
    ).service(
        web::scope("/products")
        .service(
            web::resource("")
            .route(web::get().to(product::get_products))
            .route(web::post().to(product::add_product)),
        ),
    );   
}