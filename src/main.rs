use std::env;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file is not found");
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL is not set");

    let db_coon = Database::connect(&db_url).await.expect("Can't connect to the database");

    db_coon.ping().await.expect("Failed to ping the database");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: "actix demo".to_string(),
            }))
            .wrap(actix_web::middleware::Logger::default())
            .service(hello)
    })
        .bind(("127.0.0.1", 8067))
        .unwrap()
        .run()
        .await
        .unwrap();
}

#[get("/hello")]
async fn hello(app_state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! This is {}.", &app_state.app_name))
}

struct AppState {
    pub app_name: String,
}
