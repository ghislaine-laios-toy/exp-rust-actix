use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{anyhow, Context};
use log::info;
use sea_orm::Database;
use exp_rust_actix::app_state::AppState;

use exp_rust_actix::services;
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

    // Since the ping operation may take a considerable amount of time,
    // it's essential to inform the user once the ping has been successfully completed.
    info!("Connected to the database.");

    // Confirm the application of pending migrations.
    let pending_migrations = Migrator::get_pending_migrations(&db_coon)
        .await
        .expect("Failed to get the pending migrations");
    if !pending_migrations.is_empty() {
        return Err(anyhow!("Pending migrations await application."));
    }

    // Construct the AppState entity.
    let app_state = web::Data::new(AppState {
        app_name: "actix demo".to_string(),
        db_coon,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(hello)
            .configure(services::author::init)
            .configure(services::error_exp::init)
    })
        .bind(("127.0.0.1", 8001))
        .context("Fail to bind to 127.0.0.1:8001")?
        .run()
        .await
        .unwrap();

    Ok(())
}

#[get("/hello")]
async fn hello(app_state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! This is {}.", &app_state.app_name))
}

