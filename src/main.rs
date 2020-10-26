extern crate log;

use actix_web::{web, middleware, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::mssql::MssqlPool;
use std::env;

mod todo;

async fn heartbeat() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = MssqlPool::connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .route("/heartbeat", web::get().to(heartbeat))
            .configure(todo::init)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
