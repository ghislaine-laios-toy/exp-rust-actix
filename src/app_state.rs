use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub db_coon: DatabaseConnection,
}
