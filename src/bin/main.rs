use actix_web::{middleware,App,HttpServer};
use actix_web_test::app_config::config_app;
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    let server = HttpServer::new(||{
        App::new()
        .configure(config_app)
        .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1",8082))?
    .run();
    println!("Server running...");
    server.await
    
}