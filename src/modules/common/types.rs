use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, ToSql, Output};
use std::io::Write;
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Import SQL types from schema
use crate::schema::sql_types::{TipoPerfil as TipoPerfilSql, TipoMovimiento as TipoMovimientoSql};

// Enum for TipoPerfil
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow, ToSchema)]
#[diesel(sql_type = TipoPerfilSql)]
#[schema(example = "CLIENTE")]
pub enum TipoPerfil {
    #[serde(rename = "VENDEDOR")]
    Vendedor,
    #[serde(rename = "CLIENTE")]
    Cliente,
    #[serde(rename = "PROVEEDOR")]
    Proveedor,
}

impl ToSql<TipoPerfilSql, Pg> for TipoPerfil {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TipoPerfil::Vendedor => out.write_all(b"VENDEDOR")?,
            TipoPerfil::Cliente => out.write_all(b"CLIENTE")?,
            TipoPerfil::Proveedor => out.write_all(b"PROVEEDOR")?,
        }
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<TipoPerfilSql, Pg> for TipoPerfil {
    fn from_sql(bytes: diesel::pg::PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"VENDEDOR" => Ok(TipoPerfil::Vendedor),
            b"CLIENTE" => Ok(TipoPerfil::Cliente),
            b"PROVEEDOR" => Ok(TipoPerfil::Proveedor),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// Enum for TipoMovimiento
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow, ToSchema)]
#[diesel(sql_type = TipoMovimientoSql)]
#[schema(example = "ENTRADA")]
pub enum TipoMovimiento {
    #[serde(rename = "ENTRADA")]
    Entrada,
    #[serde(rename = "SALIDA")]
    Salida,
    #[serde(rename = "AJUSTE")]
    Ajuste,
}

impl ToSql<TipoMovimientoSql, Pg> for TipoMovimiento {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TipoMovimiento::Entrada => out.write_all(b"ENTRADA")?,
            TipoMovimiento::Salida => out.write_all(b"SALIDA")?,
            TipoMovimiento::Ajuste => out.write_all(b"AJUSTE")?,
        }
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<TipoMovimientoSql, Pg> for TipoMovimiento {
    fn from_sql(bytes: diesel::pg::PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"ENTRADA" => Ok(TipoMovimiento::Entrada),
            b"SALIDA" => Ok(TipoMovimiento::Salida),
            b"AJUSTE" => Ok(TipoMovimiento::Ajuste),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
