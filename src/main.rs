use std::env;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use anyhow::{anyhow, Context};
use sea_orm::{Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect(".env file is not found");
    env_logger::init();

    // Initialize the database (pool).

    let db_url =
        env::var("DATABASE_URL").context("Environment variable DATABASE_URL is not set")?;

    let db_coon = Database::connect(&db_url)
        .await
        .context("Can't connect to the database")?;

    db_coon
        .ping()
        .await
        .context("Failed to ping the database")?;

    // Confirm the application of pending migrations.
    let pending_migrations = Migrator::get_pending_migrations(&db_coon)
        .await
        .expect("Failed to get the pending migrations");
    if !pending_migrations.is_empty() {
        return Err(anyhow!("Pending migrations await application."));
    }

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: "actix demo".to_string(),
                db_coon,
            }))
            .wrap(actix_web::middleware::Logger::default())
            .service(hello)
    })
    .bind(("127.0.0.1", 8067))
    .context("Fail to bind to 127.0.0.1:8067")?
    .run()
    .await
    .unwrap();

    Ok(())
}

#[get("/hello")]
async fn hello(app_state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! This is {}.", &app_state.app_name))
}

struct AppState {
    pub app_name: String,
    pub db_coon: DatabaseConnection,
}
