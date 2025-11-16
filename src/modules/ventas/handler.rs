use actix_web::{web, HttpResponse, ResponseError, Result};
use crate::modules::ventas::model::{CrearVentaRequest, VentasQueryParams};
use crate::modules::ventas::service::VentaService;
use crate::state::app_state::AppState;

/// POST /api/ventas - RF1: Crear venta
pub async fn crear_venta(
    state: web::Data<AppState>,
    body: web::Json<CrearVentaRequest>,
) -> Result<HttpResponse> {
    let service = &state.venta_service;

    match service.procesar_venta(body.into_inner()) {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(e) => Ok(e.error_response()),
    }
}

/// GET /api/ventas - RF2: Listar ventas con filtros
pub async fn listar_ventas(
    state: web::Data<AppState>,
    query: web::Query<VentasQueryParams>,
) -> Result<HttpResponse> {
    let service = &state.venta_service;

    match service.obtener_ventas(
        query.id_cliente.clone(),
        query.sucursal.clone(),
        query.fecha_desde.clone(),
        query.fecha_hasta.clone(),
    ) {
        Ok(ventas) => Ok(HttpResponse::Ok().json(ventas)),
        Err(e) => Ok(e.error_response()),
    }
}

/// GET /api/ventas/:id - RF2: Obtener venta por ID
pub async fn obtener_venta(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let service = &state.venta_service;

    match service.obtener_venta_por_id(&id) {
        Ok(venta) => Ok(HttpResponse::Ok().json(venta)),
        Err(e) => Ok(e.error_response()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ventas")
            .route("", web::post().to(crear_venta))
            .route("", web::get().to(listar_ventas))
            .route("/{id}", web::get().to(obtener_venta))
    );
}
