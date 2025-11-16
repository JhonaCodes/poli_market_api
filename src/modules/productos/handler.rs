use actix_web::{web, HttpResponse, ResponseError, Result};
use crate::modules::productos::model::CrearProductoRequest;
use crate::state::app_state::AppState;

/// POST /api/productos - Crear nuevo producto
pub async fn crear_producto(
    state: web::Data<AppState>,
    body: web::Json<CrearProductoRequest>,
) -> Result<HttpResponse> {
    let service = &state.producto_service;

    match service.crear_producto(body.into_inner()) {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(e) => Ok(e.error_response()),
    }
}

/// GET /api/productos/:id - RF3: Obtener producto por ID con stock
pub async fn obtener_producto(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let service = &state.producto_service;

    match service.obtener_producto(&id) {
        Ok(producto) => Ok(HttpResponse::Ok().json(producto)),
        Err(e) => Ok(e.error_response()),
    }
}

/// GET /api/productos - Listar productos con stock
pub async fn listar_productos(
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let service = &state.producto_service;

    match service.listar_productos() {
        Ok(productos) => Ok(HttpResponse::Ok().json(productos)),
        Err(e) => Ok(e.error_response()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/productos")
            .route("", web::post().to(crear_producto))
            .route("", web::get().to(listar_productos))
            .route("/{id}", web::get().to(obtener_producto))
    );
}
