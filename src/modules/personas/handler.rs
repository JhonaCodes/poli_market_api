use actix_web::{web, HttpResponse, ResponseError, Result};
use serde::Deserialize;
use crate::modules::personas::service::PersonaService;
use crate::state::app_state::AppState;

#[derive(Deserialize)]
pub struct PersonasQuery {
    pub tipo: Option<String>,
}

/// GET /api/personas/:id - RF4: Obtener persona por ID
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
            .route("", web::get().to(listar_personas))
            .route("/{id}", web::get().to(obtener_persona))
    );
}
