mod config;
mod db;
mod handlers;
mod models;

use crate::config as env_config;
use crate::db::get_todo_lists;
use crate::handlers::status;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cnf = env_config::AppConfig::from_env();

    println!(
        "Running server at http://{}:{}",
        cnf.server.host.as_str(),
        cnf.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(cnf.db.clone())
            .route("/", web::get().to(status))
    })
    .bind((cnf.server.host.as_str(), cnf.server.port))?
    .run()
    .await
}
