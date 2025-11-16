use actix_web::{web, HttpResponse, ResponseError, Result};
use serde::Deserialize;
use utoipa::{ToSchema, IntoParams};
use crate::modules::personas::model::{CrearPersonaRequest, PersonaResponse, PersonaCreadaResponse};
use crate::modules::common::errors::ErrorResponse;
use crate::state::app_state::AppState;

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct PersonasQuery {
    #[schema(example = "CLIENTE")]
    pub tipo: Option<String>,
}

/// POST /api/personas - Crear nueva persona
#[utoipa::path(
    post,
    path = "/v1/personas",
    tag = "Personas",
    request_body = CrearPersonaRequest,
    responses(
        (status = 201, description = "Persona creada exitosamente", body = PersonaCreadaResponse),
        (status = 400, description = "Datos de entrada inválidos", body = ErrorResponse),
        (status = 500, description = "Error interno del servidor", body = ErrorResponse)
    )
)]
pub async fn crear_persona(
    state: web::Data<AppState>,
    body: web::Json<CrearPersonaRequest>,
) -> Result<HttpResponse> {
    let service = &state.persona_service;

    match service.crear_persona(body.into_inner()) {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(e) => Ok(e.error_response()),
    }
}

/// GET /api/personas/:id - RF4: Obtener persona por ID
#[utoipa::path(
    get,
    path = "/v1/personas/{id}",
    tag = "Personas",
    params(
        ("id" = String, Path, description = "ID de la persona (UUID)")
    ),
    responses(
        (status = 200, description = "Persona encontrada", body = PersonaResponse),
        (status = 404, description = "Persona no encontrada", body = ErrorResponse),
        (status = 500, description = "Error interno del servidor", body = ErrorResponse)
    )
)]
pub async fn obtener_persona(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let service = &state.persona_service;

    match service.obtener_persona(&id) {
        Ok(persona) => Ok(HttpResponse::Ok().json(persona)),
        Err(e) => Ok(e.error_response()),
    }
}

/// GET /api/personas - Listar personas
#[utoipa::path(
    get,
    path = "/v1/personas",
    tag = "Personas",
    params(
        PersonasQuery
    ),
    responses(
        (status = 200, description = "Lista de personas", body = Vec<PersonaResponse>),
        (status = 500, description = "Error interno del servidor", body = ErrorResponse)
    )
)]
pub async fn listar_personas(
    state: web::Data<AppState>,
    query: web::Query<PersonasQuery>,
) -> Result<HttpResponse> {
    let service = &state.persona_service;

    match service.listar_personas(query.tipo.clone()) {
        Ok(personas) => Ok(HttpResponse::Ok().json(personas)),
        Err(e) => Ok(e.error_response()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/personas")
            .route("", web::post().to(crear_persona))
            .route("", web::get().to(listar_personas))
            .route("/{id}", web::get().to(obtener_persona))
    );
}
