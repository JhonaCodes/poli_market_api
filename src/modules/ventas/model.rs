use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{ToSchema, IntoParams};
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
#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearVentaRequest {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id_cliente: String,
    #[schema(example = "Bogotá Centro")]
    pub sucursal: Option<String>,
    pub detalles: Vec<DetalleVentaRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DetalleVentaRequest {
    #[schema(example = "660e8400-e29b-41d4-a716-446655440000")]
    pub id_producto: String,
    #[schema(example = 2)]
    pub cantidad: i32,
}

// DTO for sale response
#[derive(Debug, Serialize, ToSchema)]
pub struct VentaResponse {
    #[schema(example = "770e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id_cliente: String,
    #[schema(example = "2025-11-17T10:30:00")]
    pub fecha: String,
    #[schema(example = 2400000.0)]
    pub total: f64,
    #[schema(example = "Bogotá Centro")]
    pub sucursal: Option<String>,
    pub detalles: Vec<DetalleVentaResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DetalleVentaResponse {
    #[schema(example = "660e8400-e29b-41d4-a716-446655440000")]
    pub id_producto: String,
    #[schema(example = "Laptop Dell Inspiron 15")]
    pub nombre_producto: String,
    #[schema(example = 2)]
    pub cantidad: i32,
    #[schema(example = 1200000.0)]
    pub precio_unitario: f64,
    #[schema(example = 2400000.0)]
    pub subtotal: f64,
}

// DTO for created sale response
#[derive(Debug, Serialize, ToSchema)]
pub struct VentaCreadaResponse {
    #[schema(example = "770e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "Venta procesada exitosamente")]
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
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct VentasQueryParams {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id_cliente: Option<String>,
    #[schema(example = "Bogotá Centro")]
    pub sucursal: Option<String>,
    #[schema(example = "2025-11-01")]
    pub fecha_desde: Option<String>,
    #[schema(example = "2025-11-30")]
    pub fecha_hasta: Option<String>,
}
