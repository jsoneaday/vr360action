pub mod repo {
    pub mod base;
    pub mod pc;
}
pub mod app_state;
pub mod pc_route;
pub mod user_error;

use actix_cors::Cors;
use actix_web::{HttpServer, http::header, App, web};
use app_state::AppState;
use pc_route::get_all_pc_info;
use repo::base::{DbRepo, Repository};

pub async fn run() -> std::io::Result<()> {
    let host = "0.0.0.0"; // env::var("HOST").unwrap();
    let port: u16 = 5002; // env::var("PORT").unwrap().parse::<u16>().unwrap();
    let allowed_domain = "http://localhost:5173"; // env::var("ALLOWED_DOMAIN").unwrap();
    
    let app_data = actix_web::web::Data::new(AppState{
        repo: DbRepo::init().await
    });    

    HttpServer::new(move || {
        App::new() 
            .app_data(app_data.clone())
            .wrap(
                Cors::default()
                    .allowed_origin(&allowed_domain)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        header::CONTENT_TYPE,
                        header::AUTHORIZATION,
                        header::ACCEPT,
                    ])
                    //.supports_credentials()
                    .max_age(3600)
            )                              
            .service(
                web::scope("/v1")
                    .service(web::resource("/pc_info")
                        .route(web::get().to(get_all_pc_info))

            ))            
    })
    .bind((host, port)).expect("")
    .run()
    .await
}