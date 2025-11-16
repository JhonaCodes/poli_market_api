use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::common::types::TipoPerfil;
use crate::modules::personas::model::{Persona, PersonaResponse};
use crate::modules::personas::repository::PersonaRepository;

pub struct PersonaService {
    repository: PersonaRepository,
}

impl PersonaService {
    pub fn new(repository: PersonaRepository) -> Self {
        PersonaService { repository }
    }

    /// RF4: Obtener persona por ID
    pub fn obtener_persona(&self, id_str: &str) -> ApiResult<PersonaResponse> {
        let id = Uuid::parse_str(id_str)
            .map_err(|_| ApiError::InvalidInput("ID de persona inválido".to_string()))?;

        let persona = self.repository.buscar_por_id(id)?;
        Ok(PersonaResponse::from(persona))
    }

    /// Listar personas con filtro opcional por tipo de perfil
    pub fn listar_personas(&self, tipo_perfil: Option<String>) -> ApiResult<Vec<PersonaResponse>> {
        let perfil_filtro = match tipo_perfil {
            Some(tipo) => {
                let perfil = match tipo.to_uppercase().as_str() {
                    "VENDEDOR" => TipoPerfil::Vendedor,
                    "CLIENTE" => TipoPerfil::Cliente,
                    "PROVEEDOR" => TipoPerfil::Proveedor,
                    _ => return Err(ApiError::InvalidInput("Tipo de perfil inválido".to_string())),
                };
                Some(perfil)
            }
            None => None,
        };

        let personas = self.repository.listar(perfil_filtro)?;
        Ok(personas.into_iter().map(PersonaResponse::from).collect())
    }

    /// Validar que una persona existe y está activa
    pub fn validar_persona_activa(&self, id: Uuid) -> ApiResult<()> {
        let es_activo = self.repository.validar_activo(id)?;
        if !es_activo {
            return Err(ApiError::InactiveClient);
        }
        Ok(())
    }
}
