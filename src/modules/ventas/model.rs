use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::{ventas, detalle_ventas};

// Domain Model for Venta
#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = ventas)]
pub struct Venta {
    pub id: Uuid,
    pub id_persona: Uuid,
    pub fecha: NaiveDateTime,
    pub monto: BigDecimal,
    pub sucursal: Option<String>,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_actualizacion: NaiveDateTime,
    pub activo: bool,
}

// Domain Model for DetalleVenta
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Associations)]
#[diesel(belongs_to(Venta, foreign_key = id_venta))]
#[diesel(table_name = detalle_ventas)]
pub struct DetalleVenta {
    pub id: Uuid,
    pub id_venta: Uuid,
    pub id_producto: Uuid,
    pub cantidad: i32,
    pub monto: BigDecimal,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_actualizacion: NaiveDateTime,
    pub activo: bool,
}

// DTO for creating a sale
#[derive(Debug, Deserialize)]
pub struct CrearVentaRequest {
    pub id_cliente: String,
    pub sucursal: Option<String>,
    pub detalles: Vec<DetalleVentaRequest>,
}

#[derive(Debug, Deserialize)]
pub struct DetalleVentaRequest {
    pub id_producto: String,
    pub cantidad: i32,
}

// DTO for sale response
#[derive(Debug, Serialize)]
pub struct VentaResponse {
    pub id: String,
    pub id_cliente: String,
    pub fecha: String,
    pub total: f64,
    pub sucursal: Option<String>,
    pub detalles: Vec<DetalleVentaResponse>,
}

#[derive(Debug, Serialize)]
pub struct DetalleVentaResponse {
    pub id_producto: String,
    pub nombre_producto: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

// DTO for created sale response
#[derive(Debug, Serialize)]
pub struct VentaCreadaResponse {
    pub id: String,
    pub mensaje: String,
}

// Insertable structs for database
#[derive(Debug, Insertable)]
#[diesel(table_name = ventas)]
pub struct NuevaVenta {
    pub id: Uuid,
    pub id_persona: Uuid,
    pub fecha: NaiveDateTime,
    pub monto: BigDecimal,
    pub sucursal: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = detalle_ventas)]
pub struct NuevoDetalleVenta {
    pub id: Uuid,
    pub id_venta: Uuid,
    pub id_producto: Uuid,
    pub cantidad: i32,
    pub monto: BigDecimal,
}

// Query parameters for filtering sales
#[derive(Debug, Deserialize)]
pub struct VentasQueryParams {
    pub id_cliente: Option<String>,
    pub sucursal: Option<String>,
    pub fecha_desde: Option<String>,
    pub fecha_hasta: Option<String>,
}
