use actix_web::{web, HttpResponse, ResponseError, Result};
use crate::modules::inventarios::service::InventarioService;
use crate::state::app_state::AppState;

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
            .route("/disponibilidad/{id}", web::get().to(obtener_disponibilidad))
    );
}
