use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use crate::schema::productos;

// Domain Model (Database Entity)
#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = productos)]
pub struct Producto {
    pub id: Uuid,
    pub nombre: String,
    pub cantidad: i32,
    pub unidad_venta: String,
    pub precio_unitario: BigDecimal,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_actualizacion: NaiveDateTime,
    pub activo: bool,
}

// DTO for API Response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductoResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "Laptop Dell Inspiron 15")]
    pub nombre: String,
    #[schema(example = 1200000.0)]
    pub precio_unitario: f64,
    #[schema(example = "unidad")]
    pub unidad_venta: String,
    #[schema(example = 50)]
    pub stock_actual: i32,
}

// DTO for creating a new Producto (database insert)
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = productos)]
pub struct NuevoProducto {
    pub nombre: String,
    pub cantidad: i32,
    pub unidad_venta: String,
    pub precio_unitario: BigDecimal,
}

// DTO for producto creation request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CrearProductoRequest {
    #[schema(example = "Laptop Dell Inspiron 15")]
    pub nombre: String,
    #[schema(example = 50)]
    pub cantidad: i32,
    #[schema(example = "unidad")]
    pub unidad_venta: String,
    #[schema(example = 1200000.0)]
    pub precio_unitario: f64,
}

// DTO for producto creation response
#[derive(Debug, Serialize, ToSchema)]
pub struct ProductoCreadoResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "Producto creado exitosamente")]
    pub mensaje: String,
}
