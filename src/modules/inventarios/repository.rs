use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use uuid::Uuid;
use chrono::Utc;
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
            .select(Inventario::as_select())
            .first(&mut conn)
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
            .select(DetalleInventario::as_select())
            .load(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    /// Crear inventario inicial para un producto nuevo
    pub fn crear_inventario_inicial(
        &self,
        conn: &mut PgConnection,
        id_producto: Uuid,
        id_persona: Uuid,
        cantidad_inicial: i32
    ) -> ApiResult<()> {
        let id_inventario = Uuid::new_v4();

        // Crear el registro en la tabla inventarios
        diesel::insert_into(inventarios::table)
            .values((
                inventarios::id.eq(id_inventario),
                inventarios::id_producto.eq(id_producto),
                inventarios::id_persona.eq(id_persona),
                inventarios::cantidad_disponible.eq(cantidad_inicial),
            ))
            .execute(conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Registrar el movimiento inicial de entrada si la cantidad es mayor a 0
        if cantidad_inicial > 0 {
            let movimiento_inicial = NuevoMovimiento {
                id_producto,
                tipo_movimiento: TipoMovimiento::Entrada,
                fecha: Utc::now().naive_utc(),
                id_persona,
                cantidad: cantidad_inicial,
                observaciones: Some("Inventario inicial".to_string()),
            };

            self.registrar_movimiento(conn, movimiento_inicial)?;
        }

        Ok(())
    }

    /// Registrar un movimiento y actualizar el stock
    pub fn registrar_movimiento_con_actualizacion(
        &self,
        id_producto: Uuid,
        tipo_movimiento: TipoMovimiento,
        id_persona: Uuid,
        cantidad: i32,
        observaciones: Option<String>
    ) -> ApiResult<Uuid> {
        let mut conn = self.get_connection()?;

        // Iniciar transacción
        conn.transaction::<Uuid, ApiError, _>(|conn| {
            // Validar que el producto tiene inventario
            let inventario_existe = inventarios::table
                .filter(inventarios::id_producto.eq(id_producto))
                .filter(inventarios::activo.eq(true))
                .count()
                .get_result::<i64>(conn)
                .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

            if inventario_existe == 0 {
                return Err(ApiError::NotFound(
                    format!("No existe inventario para el producto {}", id_producto)
                ));
            }

            // Calcular el cambio en el stock según el tipo de movimiento
            let cambio_stock = match tipo_movimiento {
                TipoMovimiento::Entrada => cantidad,
                TipoMovimiento::Salida => -cantidad,
                TipoMovimiento::Ajuste => cantidad, // El ajuste puede ser positivo o negativo
            };

            // Validar que no quede stock negativo
            let stock_actual = self.obtener_stock_con_conexion(conn, id_producto)?;
            let nuevo_stock = stock_actual + cambio_stock;

            if nuevo_stock < 0 {
                return Err(ApiError::BusinessRuleViolation(
                    format!("Stock insuficiente. Stock actual: {}, Cambio solicitado: {}",
                            stock_actual, cambio_stock)
                ));
            }

            // Actualizar el stock
            self.actualizar_stock(conn, id_producto, cambio_stock)?;

            // Registrar el movimiento
            let movimiento = NuevoMovimiento {
                id_producto,
                tipo_movimiento,
                fecha: Utc::now().naive_utc(),
                id_persona,
                cantidad,
                observaciones,
            };

            let id = self.registrar_movimiento(conn, movimiento)?;

            Ok(id)
        })
    }

    /// Obtener stock usando una conexión existente (para uso en transacciones)
    fn obtener_stock_con_conexion(&self, conn: &mut PgConnection, id_producto: Uuid) -> ApiResult<i32> {
        let inventario = inventarios::table
            .filter(inventarios::id_producto.eq(id_producto))
            .filter(inventarios::activo.eq(true))
            .select(Inventario::as_select())
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::NotFound(
                    format!("Inventario para producto {} no encontrado", id_producto)
                ),
                _ => ApiError::DatabaseError(e.to_string()),
            })?;

        Ok(inventario.cantidad_disponible)
    }
}
