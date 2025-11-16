use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use crate::modules::common::types::TipoPerfil;
use crate::schema::personas;

// Domain Model (Database Entity)
#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = personas)]
pub struct Persona {
    pub id: Uuid,
    pub nombre: String,
    pub documento: String,
    pub perfil: TipoPerfil,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_actualizacion: NaiveDateTime,
    pub activo: bool,
}

// DTO for API Response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PersonaResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "Juan Pérez")]
    pub nombre: String,
    #[schema(example = "1234567890")]
    pub documento: String,
    #[schema(example = "CLIENTE")]
    pub perfil: String,
    #[schema(example = "juan.perez@example.com")]
    pub email: Option<String>,
    #[schema(example = "+57 300 123 4567")]
    pub telefono: Option<String>,
}

impl From<Persona> for PersonaResponse {
    fn from(persona: Persona) -> Self {
        PersonaResponse {
            id: persona.id.to_string(),
            nombre: persona.nombre,
            documento: persona.documento,
            perfil: format!("{:?}", persona.perfil).to_uppercase(),
            email: persona.email,
            telefono: persona.telefono,
        }
    }
}

// DTO for creating a new Persona (database insert)
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = personas)]
pub struct NuevaPersona {
    pub nombre: String,
    pub documento: String,
    pub perfil: TipoPerfil,
    pub email: Option<String>,
    pub telefono: Option<String>,
}

// DTO for persona creation request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearPersonaRequest {
    #[schema(example = "Juan Pérez")]
    pub nombre: String,
    #[schema(example = "1234567890")]
    pub documento: String,
    #[schema(example = "CLIENTE")]
    pub perfil: String,
    #[schema(example = "juan.perez@example.com")]
    pub email: Option<String>,
    #[schema(example = "+57 300 123 4567")]
    pub telefono: Option<String>,
}

// DTO for persona creation response
#[derive(Debug, Serialize, ToSchema)]
pub struct PersonaCreadaResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "Persona creada exitosamente")]
    pub mensaje: String,
}
