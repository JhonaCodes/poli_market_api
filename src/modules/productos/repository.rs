use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::productos::model::Producto;
use crate::schema::productos;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct ProductoRepository {
    pool: DbPool,
}

impl ProductoRepository {
    pub fn new(pool: DbPool) -> Self {
        ProductoRepository { pool }
    }

    fn get_connection(&self) -> ApiResult<DbConnection> {
        self.pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub fn buscar_por_id(&self, id: Uuid) -> ApiResult<Producto> {
        let mut conn = self.get_connection()?;

        productos::table
            .find(id)
            .filter(productos::activo.eq(true))
            .first::<Producto>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::ProductNotFound,
                _ => ApiError::DatabaseError(e.to_string()),
            })
    }

    pub fn listar(&self) -> ApiResult<Vec<Producto>> {
        let mut conn = self.get_connection()?;

        productos::table
            .filter(productos::activo.eq(true))
            .order(productos::nombre.asc())
            .load::<Producto>(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub fn verificar_existe_y_activo(&self, id: Uuid) -> ApiResult<bool> {
        let mut conn = self.get_connection()?;

        let count: i64 = productos::table
            .find(id)
            .filter(productos::activo.eq(true))
            .count()
            .get_result(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(count > 0)
    }
}
