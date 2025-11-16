use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::common::types::TipoPerfil;
use crate::modules::personas::model::Persona;
use crate::schema::personas;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct PersonaRepository {
    pool: DbPool,
}

impl PersonaRepository {
    pub fn new(pool: DbPool) -> Self {
        PersonaRepository { pool }
    }

    fn get_connection(&self) -> ApiResult<DbConnection> {
        self.pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub fn buscar_por_id(&self, id: Uuid) -> ApiResult<Persona> {
        let mut conn = self.get_connection()?;

        personas::table
            .find(id)
            .filter(personas::activo.eq(true))
            .first::<Persona>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::NotFound(format!("Persona con ID {} no encontrada", id)),
                _ => ApiError::DatabaseError(e.to_string()),
            })
    }

    pub fn listar(&self, perfil_filtro: Option<TipoPerfil>) -> ApiResult<Vec<Persona>> {
        let mut conn = self.get_connection()?;

        let mut query = personas::table
            .filter(personas::activo.eq(true))
            .into_boxed();

        if let Some(perfil) = perfil_filtro {
            query = query.filter(personas::perfil.eq(perfil));
        }

        query
            .load::<Persona>(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub fn validar_activo(&self, id: Uuid) -> ApiResult<bool> {
        let persona = self.buscar_por_id(id)?;
        Ok(persona.activo)
    }
}
