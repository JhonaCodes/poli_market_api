use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use crate::modules::common::types::TipoMovimiento;
use crate::schema::{inventarios, detalle_inventarios};

// Domain Model for Inventario
#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = inventarios)]
pub struct Inventario {
    pub id: Uuid,
    pub id_producto: Uuid,
    pub id_persona: Uuid,
    pub cantidad_disponible: i32,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_actualizacion: NaiveDateTime,
    pub activo: bool,
}

// Domain Model for DetalleInventario
#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = detalle_inventarios)]
pub struct DetalleInventario {
    pub id: Uuid,
    pub id_producto: Uuid,
    pub tipo_movimiento: TipoMovimiento,
    pub fecha: NaiveDateTime,
    pub id_persona: Uuid,
    pub cantidad: i32,
    pub observaciones: Option<String>,
    pub fecha_creacion: NaiveDateTime,
    pub fecha_actualizacion: NaiveDateTime,
    pub activo: bool,
}

// DTO for stock availability response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DisponibilidadResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id_producto: String,
    #[schema(example = 45)]
    pub cantidad_disponible: i32,
}

// DTO for creating a new inventory movement
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = detalle_inventarios)]
pub struct NuevoMovimiento {
    pub id_producto: Uuid,
    pub tipo_movimiento: TipoMovimiento,
    pub fecha: NaiveDateTime,
    pub id_persona: Uuid,
    pub cantidad: i32,
    pub observaciones: Option<String>,
}

// DTO for movement request
#[derive(Debug, Deserialize, ToSchema)]
pub struct MovimientoRequest {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id_producto: String,
    #[schema(example = "ENTRADA")]
    pub tipo_movimiento: String,
    #[schema(example = "660e8400-e29b-41d4-a716-446655440000")]
    pub id_persona: String,
    #[schema(example = 20)]
    pub cantidad: i32,
    #[schema(example = "Compra de inventario mensual")]
    pub observaciones: Option<String>,
}

// DTO for movement response
#[derive(Debug, Serialize, ToSchema)]
pub struct MovimientoRegistradoResponse {
    #[schema(example = "770e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "Movimiento registrado exitosamente")]
    pub mensaje: String,
}
