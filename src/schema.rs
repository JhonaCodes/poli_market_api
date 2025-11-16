// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tipo_movimiento"))]
    pub struct TipoMovimiento;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tipo_perfil"))]
    pub struct TipoPerfil;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TipoMovimiento;

    detalle_inventarios (id) {
        id -> Uuid,
        id_producto -> Uuid,
        tipo_movimiento -> TipoMovimiento,
        fecha -> Timestamp,
        id_persona -> Uuid,
        cantidad -> Int4,
        observaciones -> Nullable<Text>,
        fecha_creacion -> Timestamp,
        fecha_actualizacion -> Timestamp,
        activo -> Bool,
    }
}

diesel::table! {
    detalle_ventas (id) {
        id -> Uuid,
        id_venta -> Uuid,
        id_producto -> Uuid,
        cantidad -> Int4,
        monto -> Numeric,
        fecha_creacion -> Timestamp,
        fecha_actualizacion -> Timestamp,
        activo -> Bool,
    }
}

diesel::table! {
    inventarios (id) {
        id -> Uuid,
        id_producto -> Uuid,
        id_persona -> Uuid,
        cantidad_disponible -> Int4,
        fecha_creacion -> Timestamp,
        fecha_actualizacion -> Timestamp,
        activo -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TipoPerfil;

    personas (id) {
        id -> Uuid,
        #[max_length = 255]
        nombre -> Varchar,
        #[max_length = 50]
        documento -> Varchar,
        perfil -> TipoPerfil,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 20]
        telefono -> Nullable<Varchar>,
        fecha_creacion -> Timestamp,
        fecha_actualizacion -> Timestamp,
        activo -> Bool,
    }
}

diesel::table! {
    productos (id) {
        id -> Uuid,
        #[max_length = 255]
        nombre -> Varchar,
        cantidad -> Int4,
        #[max_length = 50]
        unidad_venta -> Varchar,
        precio_unitario -> Numeric,
        fecha_creacion -> Timestamp,
        fecha_actualizacion -> Timestamp,
        activo -> Bool,
    }
}

diesel::table! {
    ventas (id) {
        id -> Uuid,
        id_persona -> Uuid,
        fecha -> Timestamp,
        monto -> Numeric,
        #[max_length = 100]
        sucursal -> Nullable<Varchar>,
        fecha_creacion -> Timestamp,
        fecha_actualizacion -> Timestamp,
        activo -> Bool,
    }
}

diesel::joinable!(detalle_inventarios -> personas (id_persona));
diesel::joinable!(detalle_inventarios -> productos (id_producto));
diesel::joinable!(detalle_ventas -> productos (id_producto));
diesel::joinable!(detalle_ventas -> ventas (id_venta));
diesel::joinable!(inventarios -> personas (id_persona));
diesel::joinable!(inventarios -> productos (id_producto));
diesel::joinable!(ventas -> personas (id_persona));

diesel::allow_tables_to_appear_in_same_query!(
    detalle_inventarios,
    detalle_ventas,
    inventarios,
    personas,
    productos,
    ventas,
);
