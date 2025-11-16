use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::productos::model::{Producto, NuevoProducto};
use crate::schema::productos;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct ProductoRepository {
    pub pool: DbPool,
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
            .select(Producto::as_select())
            .first(&mut conn)
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
            .select(Producto::as_select())
            .load(&mut conn)
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

    pub fn crear(&self, nuevo_producto: NuevoProducto) -> ApiResult<Uuid> {
        let mut conn = self.get_connection()?;

        // Validar que el nombre no esté vacío
        if nuevo_producto.nombre.trim().is_empty() {
            return Err(ApiError::InvalidInput("El nombre del producto es requerido".to_string()));
        }

        let id = Uuid::new_v4();

        // Insertar el nuevo producto con ID generado
        diesel::insert_into(productos::table)
            .values((
                productos::id.eq(id),
                productos::nombre.eq(&nuevo_producto.nombre),
                productos::cantidad.eq(&nuevo_producto.cantidad),
                productos::unidad_venta.eq(&nuevo_producto.unidad_venta),
                productos::precio_unitario.eq(&nuevo_producto.precio_unitario),
            ))
            .execute(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(id)
    }

    /// Método usado internamente en transacciones
    pub fn crear_con_conexion(&self, conn: &mut PgConnection, id: Uuid, nuevo_producto: NuevoProducto) -> ApiResult<()> {
        diesel::insert_into(productos::table)
            .values((
                productos::id.eq(id),
                productos::nombre.eq(&nuevo_producto.nombre),
                productos::cantidad.eq(&nuevo_producto.cantidad),
                productos::unidad_venta.eq(&nuevo_producto.unidad_venta),
                productos::precio_unitario.eq(&nuevo_producto.precio_unitario),
            ))
            .execute(conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
