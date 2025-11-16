use actix_web::{web, HttpResponse, ResponseError, Result};
use crate::modules::inventarios::model::{MovimientoRequest, MovimientoRegistradoResponse, DisponibilidadResponse};
use crate::modules::common::errors::ErrorResponse;
use crate::state::app_state::AppState;

/// POST /api/inventario/movimientos - Registrar movimiento de inventario
#[utoipa::path(
    post,
    path = "/v1/inventario/movimientos",
    tag = "Inventario",
    request_body = MovimientoRequest,
    responses(
        (status = 201, description = "Movimiento de inventario registrado exitosamente", body = MovimientoRegistradoResponse),
        (status = 400, description = "Datos de entrada inválidos o producto no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno del servidor", body = ErrorResponse)
    )
)]
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
#[utoipa::path(
    get,
    path = "/v1/inventario/disponibilidad/{id}",
    tag = "Inventario",
    params(
        ("id" = String, Path, description = "ID del producto (UUID)")
    ),
    responses(
        (status = 200, description = "Disponibilidad del producto consultada exitosamente", body = DisponibilidadResponse),
        (status = 404, description = "Producto no encontrado", body = ErrorResponse),
        (status = 500, description = "Error interno del servidor", body = ErrorResponse)
    )
)]
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
