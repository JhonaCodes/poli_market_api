use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::ventas::model::{Venta, DetalleVenta, NuevaVenta, NuevoDetalleVenta};
use crate::schema::{ventas, detalle_ventas};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct VentaRepository {
    pool: DbPool,
}

impl VentaRepository {
    pub fn new(pool: DbPool) -> Self {
        VentaRepository { pool }
    }

    fn get_connection(&self) -> ApiResult<DbConnection> {
        self.pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub fn guardar_con_detalles(
        &self,
        venta: NuevaVenta,
        detalles: Vec<NuevoDetalleVenta>,
    ) -> ApiResult<Uuid> {
        let mut conn = self.get_connection()?;

        // Use a transaction to ensure atomicity
        conn.transaction(|conn| {
            // Insert sale header
            diesel::insert_into(ventas::table)
                .values(&venta)
                .execute(conn)?;

            // Insert sale details
            for detalle in detalles {
                diesel::insert_into(detalle_ventas::table)
                    .values(&detalle)
                    .execute(conn)?;
            }

            Ok(venta.id)
        })
        .map_err(|e: diesel::result::Error| ApiError::DatabaseError(e.to_string()))
    }

    pub fn buscar_por_id(&self, id: Uuid) -> ApiResult<(Venta, Vec<DetalleVenta>)> {
        let mut conn = self.get_connection()?;

        let venta = ventas::table
            .find(id)
            .filter(ventas::activo.eq(true))
            .first::<Venta>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::NotFound(format!("Venta con ID {} no encontrada", id)),
                _ => ApiError::DatabaseError(e.to_string()),
            })?;

        let detalles = detalle_ventas::table
            .filter(detalle_ventas::id_venta.eq(id))
            .filter(detalle_ventas::activo.eq(true))
            .load::<DetalleVenta>(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok((venta, detalles))
    }

    pub fn listar(
        &self,
        id_cliente: Option<Uuid>,
        sucursal: Option<String>,
        fecha_desde: Option<NaiveDateTime>,
        fecha_hasta: Option<NaiveDateTime>,
    ) -> ApiResult<Vec<Venta>> {
        let mut conn = self.get_connection()?;

        let mut query = ventas::table
            .filter(ventas::activo.eq(true))
            .into_boxed();

        if let Some(cliente) = id_cliente {
            query = query.filter(ventas::id_persona.eq(cliente));
        }

        if let Some(suc) = sucursal {
            query = query.filter(ventas::sucursal.eq(suc));
        }

        if let Some(desde) = fecha_desde {
            query = query.filter(ventas::fecha.ge(desde));
        }

        if let Some(hasta) = fecha_hasta {
            query = query.filter(ventas::fecha.le(hasta));
        }

        query
            .order(ventas::fecha.desc())
            .load::<Venta>(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    pub fn obtener_detalles(&self, id_venta: Uuid) -> ApiResult<Vec<DetalleVenta>> {
        let mut conn = self.get_connection()?;

        detalle_ventas::table
            .filter(detalle_ventas::id_venta.eq(id_venta))
            .filter(detalle_ventas::activo.eq(true))
            .load::<DetalleVenta>(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }
}
