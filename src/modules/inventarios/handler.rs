use actix_web::{web, HttpResponse, ResponseError, Result};
use crate::modules::inventarios::model::MovimientoRequest;
use crate::state::app_state::AppState;

/// POST /api/inventario/movimientos - Registrar movimiento de inventario
pub async fn registrar_movimiento(
    state: web::Data<AppState>,
    body: web::Json<MovimientoRequest>,
) -> Result<HttpResponse> {
    let service = &state.inventario_service;

    match service.registrar_movimiento(body.into_inner()) {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(e) => Ok(e.error_response()),
    }
}

/// GET /api/inventario/disponibilidad/:id - RF5: Consultar disponibilidad
pub async fn obtener_disponibilidad(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let id_producto = path.into_inner();
    let service = &state.inventario_service;

    match service.obtener_disponibilidad(&id_producto) {
        Ok(disponibilidad) => Ok(HttpResponse::Ok().json(disponibilidad)),
        Err(e) => Ok(e.error_response()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/inventario")
            .route("/movimientos", web::post().to(registrar_movimiento))
            .route("/disponibilidad/{id}", web::get().to(obtener_disponibilidad))
    );
}
