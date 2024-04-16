use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use reqwest::Client;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use colored::*;

pub mod controllers;
pub mod models;
pub mod routes;
pub mod services;
pub mod middleware;

extern crate pretty_env_logger;

enum DbError {
    NoDbUrl,
    PoolCreationError,
}

impl std::fmt::Debug for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::NoDbUrl => write!(f, "No database URL found"),
            DbError::PoolCreationError => write!(f, "Failed to create pool"),
        }
    }
}

async fn handle_db_connection() -> Result<Pool<Postgres>, DbError> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").map_err(|_| DbError::NoDbUrl)?;

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .map_err(|_| DbError::PoolCreationError)?;

    println!("Database Connection Established");

    Ok(db_pool)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    pretty_env_logger::init();

    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    let db_pool = match handle_db_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to start server",
            ));
        }
    };

    // START SERVICES:
    println!("{}", "Starting Services".bright_blue().bold().underline());
    match services::init_db::db_table_check(&db_pool).await {
        Ok(()) => {
            println!("{}", "Database Tables Checked".bright_green());
        },
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to start server",
            ));
        }
    };

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        let reqwest_client = Client::new();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(middleware::auth_middleware::AuthMiddleware)
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(reqwest_client.clone()))
            .configure(routes::auth_rotues::init_routes)
    });

    let server_address = format!("127.0.0.1:{}", port);
    println!("Server running at http://{}", &server_address); // Improved log message

    server
        .bind(server_address)
        .map_err(|e| {
            println!("Error Binding: {:?}", e);
            e
        })?
        .run()
        .await
        .map_err(|e| {
            println!("Failed to start server");
            e
        })
}
