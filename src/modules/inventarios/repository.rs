use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::common::types::TipoMovimiento;
use crate::modules::inventarios::model::{Inventario, DetalleInventario, NuevoMovimiento};
use crate::schema::{inventarios, detalle_inventarios};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct InventarioRepository {
    pool: DbPool,
}

impl InventarioRepository {
    pub fn new(pool: DbPool) -> Self {
        InventarioRepository { pool }
    }

    fn get_connection(&self) -> ApiResult<DbConnection> {
        self.pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub fn obtener_stock(&self, id_producto: Uuid) -> ApiResult<i32> {
        let mut conn = self.get_connection()?;

        let inventario = inventarios::table
            .filter(inventarios::id_producto.eq(id_producto))
            .filter(inventarios::activo.eq(true))
            .first::<Inventario>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::NotFound(format!("Inventario para producto {} no encontrado", id_producto)),
                _ => ApiError::DatabaseError(e.to_string()),
            })?;

        Ok(inventario.cantidad_disponible)
    }

    pub fn validar_stock(&self, id_producto: Uuid, cantidad_requerida: i32) -> ApiResult<bool> {
        let stock_actual = self.obtener_stock(id_producto)?;
        Ok(stock_actual >= cantidad_requerida)
    }

    pub fn actualizar_stock(&self, conn: &mut PgConnection, id_producto: Uuid, cantidad: i32) -> ApiResult<()> {
        diesel::update(inventarios::table)
            .filter(inventarios::id_producto.eq(id_producto))
            .set(inventarios::cantidad_disponible.eq(inventarios::cantidad_disponible + cantidad))
            .execute(conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn registrar_movimiento(&self, conn: &mut PgConnection, movimiento: NuevoMovimiento) -> ApiResult<Uuid> {
        let id = Uuid::new_v4();

        diesel::insert_into(detalle_inventarios::table)
            .values(&movimiento)
            .execute(conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(id)
    }

    pub fn listar_movimientos(&self, id_producto: Uuid) -> ApiResult<Vec<DetalleInventario>> {
        let mut conn = self.get_connection()?;

        detalle_inventarios::table
            .filter(detalle_inventarios::id_producto.eq(id_producto))
            .filter(detalle_inventarios::activo.eq(true))
            .order(detalle_inventarios::fecha.desc())
            .load::<DetalleInventario>(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }
}
