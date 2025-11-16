use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
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
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonaResponse {
    pub id: String,
    pub nombre: String,
    pub documento: String,
    pub perfil: String,
    pub email: Option<String>,
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

// DTO for creating a new Persona
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = personas)]
pub struct NuevaPersona {
    pub nombre: String,
    pub documento: String,
    pub perfil: TipoPerfil,
    pub email: Option<String>,
    pub telefono: Option<String>,
}
