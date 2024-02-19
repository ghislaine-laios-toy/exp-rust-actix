use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() {
    env_logger::init();

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
