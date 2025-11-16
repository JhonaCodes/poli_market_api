mod schema;
mod modules;
mod config;
mod state;

use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use log::{info, error};
use std::time::Duration;

use crate::config::Config;
use crate::state::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    info!("=== POLIMARKET API STARTING ===");

    // Load configuration
    let config = Config::from_env();
    info!("Configuration loaded:");
    info!("  - Database URL: [CONFIGURED]");
    info!("  - Server: {}:{}", config.server_host, config.server_port);
    info!("  - Pool max size: {}", config.pool_max_size);

    // Create database connection pool
    info!("Creating database connection pool...");
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);

    let pool = Pool::builder()
        .max_size(config.pool_max_size)
        .min_idle(Some(config.pool_min_idle))
        .connection_timeout(Duration::from_secs(config.pool_timeout_seconds))
        .build(manager)
        .expect("Failed to create database pool");

    info!("Database pool created successfully");

    // Test database connection
    match pool.get() {
        Ok(_) => info!("Database connection test: OK"),
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            panic!("Cannot start server without database connection");
        }
    }

    // Create application state with all services
    info!("Initializing application state with services...");
    let app_state = web::Data::new(AppState::new(pool));
    info!("Application state initialized successfully");

    // Start HTTP server
    let server_host = config.server_host.clone();
    let server_port = config.server_port;

    info!("Starting HTTP server on {}:{}...", server_host, server_port);

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    // Health check endpoint
                    .route("/health", web::get().to(health_check))
                    // Module routes
                    .configure(modules::personas::handler::configure)
                    .configure(modules::productos::handler::configure)
                    .configure(modules::inventarios::handler::configure)
                    .configure(modules::ventas::handler::configure)
            )
    })
    .bind((server_host.as_str(), server_port))?
    .workers(2)
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "PoliMarket API",
        "version": "0.1.0"
    }))
}
