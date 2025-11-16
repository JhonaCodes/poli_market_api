use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::common::types::TipoPerfil;
use crate::modules::personas::model::{Persona, NuevaPersona};
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
            .select(Persona::as_select())
            .first(&mut conn)
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
            .select(Persona::as_select())
            .load(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub fn validar_activo(&self, id: Uuid) -> ApiResult<bool> {
        let persona = self.buscar_por_id(id)?;
        Ok(persona.activo)
    }

    pub fn crear(&self, nueva_persona: NuevaPersona) -> ApiResult<Uuid> {
        let mut conn = self.get_connection()?;

        // Validar que el documento no exista ya
        let existe = personas::table
            .filter(personas::documento.eq(&nueva_persona.documento))
            .filter(personas::activo.eq(true))
            .count()
            .get_result::<i64>(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        if existe > 0 {
            return Err(ApiError::BusinessRuleViolation(
                format!("Ya existe una persona con el documento {}", nueva_persona.documento)
            ));
        }

        let id = Uuid::new_v4();

        // Insertar la nueva persona con ID generado
        diesel::insert_into(personas::table)
            .values((
                personas::id.eq(id),
                personas::nombre.eq(&nueva_persona.nombre),
                personas::documento.eq(&nueva_persona.documento),
                personas::perfil.eq(&nueva_persona.perfil),
                personas::email.eq(&nueva_persona.email),
                personas::telefono.eq(&nueva_persona.telefono),
            ))
            .execute(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(id)
    }
}
