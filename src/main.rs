mod schema;
mod modules;
mod config;
mod state;

use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use log::{info, error, warn};
use std::time::Duration;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::state::app_state::AppState;

// Embed migrations into the binary
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// OpenAPI Documentation
#[derive(OpenApi)]
#[openapi(
    info(
        title = "PoliMarket API",
        version = "0.1.0",
        description = "API REST para el sistema de gestión de ventas e inventario del PoliMarket. \
        Esta API permite gestionar personas (clientes, vendedores, proveedores), productos, \
        movimientos de inventario y ventas con control automático de stock.",
        contact(
            name = "PoliMarket Team",
            email = "soporte@polimarket.com"
        )
    ),
    tags(
        (name = "Health", description = "Endpoints de verificación del estado del servicio"),
        (name = "Personas", description = "Gestión de personas (clientes, vendedores, proveedores)"),
        (name = "Productos", description = "Gestión de productos y consulta de inventario"),
        (name = "Inventario", description = "Movimientos de inventario y disponibilidad de productos"),
        (name = "Ventas", description = "Procesamiento y consulta de ventas")
    ),
    paths(
        health_check,
        modules::personas::handler::crear_persona,
        modules::personas::handler::obtener_persona,
        modules::personas::handler::listar_personas,
        modules::productos::handler::crear_producto,
        modules::productos::handler::obtener_producto,
        modules::productos::handler::listar_productos,
        modules::inventarios::handler::registrar_movimiento,
        modules::inventarios::handler::obtener_disponibilidad,
        modules::ventas::handler::crear_venta,
        modules::ventas::handler::listar_ventas,
        modules::ventas::handler::obtener_venta,
    ),
    components(
        schemas(
            // Common types
            modules::common::errors::ErrorResponse,
            modules::common::types::TipoPerfil,
            modules::common::types::TipoMovimiento,
            // Personas
            modules::personas::model::CrearPersonaRequest,
            modules::personas::model::PersonaResponse,
            modules::personas::model::PersonaCreadaResponse,
            modules::personas::handler::PersonasQuery,
            // Productos
            modules::productos::model::CrearProductoRequest,
            modules::productos::model::ProductoResponse,
            modules::productos::model::ProductoCreadoResponse,
            // Inventarios
            modules::inventarios::model::MovimientoRequest,
            modules::inventarios::model::MovimientoRegistradoResponse,
            modules::inventarios::model::DisponibilidadResponse,
            // Ventas
            modules::ventas::model::CrearVentaRequest,
            modules::ventas::model::DetalleVentaRequest,
            modules::ventas::model::VentaResponse,
            modules::ventas::model::DetalleVentaResponse,
            modules::ventas::model::VentaCreadaResponse,
            modules::ventas::model::VentasQueryParams,
        )
    )
)]
struct ApiDoc;

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

    let mut conn = loop {
        match pool.get() {
            Ok(conn) => {
                info!("✓ Database connection test: OK");
                break conn;
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
    };

    // Run pending migrations automatically
    info!("🔄 Running database migrations...");
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(versions) => {
            if versions.is_empty() {
                info!("✓ Database schema is up to date (no pending migrations)");
            } else {
                info!("✓ Successfully applied {} migration(s):", versions.len());
                for version in versions {
                    info!("   - {}", version);
                }
            }
        }
        Err(e) => {
            error!("✗ Failed to run migrations: {}", e);
            warn!("  Attempting to continue anyway (migrations may already be applied)");
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

    // Generate OpenAPI spec
    let openapi = ApiDoc::openapi();

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
            // Swagger UI
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
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
    info!("   POST /v1/personas");
    info!("   GET  /v1/personas");
    info!("   GET  /v1/personas/{{id}}");
    info!("   POST /v1/productos");
    info!("   GET  /v1/productos");
    info!("   GET  /v1/productos/{{id}}");
    info!("   POST /v1/inventario/movimientos");
    info!("   GET  /v1/inventario/disponibilidad/{{id}}");
    info!("   POST /v1/ventas");
    info!("   GET  /v1/ventas");
    info!("   GET  /v1/ventas/{{id}}");
    info!("");
    info!("📚 API Documentation:");
    info!("   Swagger UI: http://{}:{}/swagger-ui/", server_host, server_port);
    info!("   OpenAPI JSON: http://{}:{}/api-docs/openapi.json", server_host, server_port);

    server.run().await
}

/// Health check endpoint - Verifica el estado del servicio
#[utoipa::path(
    get,
    path = "/v1/health",
    tag = "Health",
    responses(
        (status = 200, description = "Servicio operativo", body = serde_json::Value,
            example = json!({
                "status": "healthy",
                "service": "PoliMarket API",
                "version": "0.1.0"
            })
        )
    )
)]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "PoliMarket API",
        "version": "0.1.0"
    }))
}
