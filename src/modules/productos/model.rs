use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductoResponse {
    pub id: String,
    pub nombre: String,
    pub precio_unitario: f64,
    pub unidad_venta: String,
    pub stock_actual: i32,
}

// DTO for creating a new Producto
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = productos)]
pub struct NuevoProducto {
    pub nombre: String,
    pub cantidad: i32,
    pub unidad_venta: String,
    pub precio_unitario: BigDecimal,
}
