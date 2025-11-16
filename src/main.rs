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

    // Initialize logger with fallback
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("=== POLIMARKET API STARTING ===");
    info!("Rust version: {}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = Config::from_env();
    info!("Configuration loaded:");
    info!("  - Database URL: [CONFIGURED]");
    info!("  - Server: {}:{}", config.server_host, config.server_port);
    info!("  - Pool max size: {}", config.pool_max_size);

    // Create database connection pool
    info!("Creating database connection pool...");
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);

    let pool = match Pool::builder()
        .max_size(config.pool_max_size)
        .min_idle(Some(config.pool_min_idle))
        .connection_timeout(Duration::from_secs(config.pool_timeout_seconds))
        .build(manager)
    {
        Ok(pool) => {
            info!("Database pool created successfully");
            pool
        }
        Err(e) => {
            error!("Failed to create database pool: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Database pool creation failed: {}", e)
            ));
        }
    };

    // Test database connection with retries
    info!("Testing database connection...");
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 5;

    loop {
        match pool.get() {
            Ok(_) => {
                info!("✓ Database connection test: OK");
                break;
            }
            Err(e) => {
                retry_count += 1;
                if retry_count >= MAX_RETRIES {
                    error!("✗ Failed to connect to database after {} attempts: {}", MAX_RETRIES, e);
                    error!("Database URL format should be: postgres://user:password@host:port/database");
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Cannot start server without database connection: {}", e)
                    ));
                }
                error!("Database connection attempt {}/{} failed: {}. Retrying in 2 seconds...",
                       retry_count, MAX_RETRIES, e);
                std::thread::sleep(Duration::from_secs(2));
            }
        }
    }

    // Create application state with all services
    info!("Initializing application state with services...");
    let app_state = web::Data::new(AppState::new(pool));
    info!("✓ Application state initialized successfully");

    // Start HTTP server
    let server_host = config.server_host.clone();
    let server_port = config.server_port;

    info!("╔═══════════════════════════════════════╗");
    info!("║  Starting HTTP server                 ║");
    info!("║  → Address: {}:{}              ║", server_host, server_port);
    info!("║  → Workers: 2                         ║");
    info!("╚═══════════════════════════════════════╝");

    info!("🌐 Configuring HTTP routes...");

    let server = HttpServer::new(move || {
        info!("🔄 Creating new worker instance...");

        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        info!("  → CORS configured (allow all origins)");

        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(cors)
            .service(
                web::scope("/v1")
                    // Health check endpoint
                    .route("/health", web::get().to(health_check))
                    // Module routes
                    .configure(modules::personas::handler::configure)
                    .configure(modules::productos::handler::configure)
                    .configure(modules::inventarios::handler::configure)
                    .configure(modules::ventas::handler::configure)
            )
    })
    .bind((server_host.as_str(), server_port))
    .map_err(|e| {
        error!("✗ Failed to bind server to {}:{} - {}", server_host, server_port, e);
        error!("  Possible causes:");
        error!("    - Port already in use");
        error!("    - Insufficient permissions");
        error!("    - Invalid host/port configuration");
        e
    })?
    .workers(2);

    info!("✓ HTTP server configured successfully");
    info!("🚀 Server is running and ready to accept connections!");
    info!("📍 Available endpoints:");
    info!("   GET  /v1/health");
    info!("   GET  /v1/personas");
    info!("   GET  /v1/personas/{{id}}");
    info!("   GET  /v1/productos");
    info!("   GET  /v1/productos/{{id}}");
    info!("   GET  /v1/inventario/disponibilidad/{{id}}");
    info!("   POST /v1/ventas");
    info!("   GET  /v1/ventas");
    info!("   GET  /v1/ventas/{{id}}");

    server.run().await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "PoliMarket API",
        "version": "0.1.0"
    }))
}
