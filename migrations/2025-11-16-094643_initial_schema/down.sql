-- ===== ELIMINAR VISTAS =====
DROP VIEW IF EXISTS vista_stock_productos;
DROP VIEW IF EXISTS vista_ventas_completas;

-- ===== ELIMINAR TRIGGERS =====
DROP TRIGGER IF EXISTS trg_actualizar_inventario_venta ON detalle_ventas;
DROP TRIGGER IF EXISTS trg_validar_stock_detalle_venta ON detalle_ventas;
DROP TRIGGER IF EXISTS trg_inventarios_actualizacion ON inventarios;
DROP TRIGGER IF EXISTS trg_ventas_actualizacion ON ventas;
DROP TRIGGER IF EXISTS trg_productos_actualizacion ON productos;
DROP TRIGGER IF EXISTS trg_personas_actualizacion ON personas;

-- ===== ELIMINAR FUNCIONES =====
DROP FUNCTION IF EXISTS actualizar_inventario_venta();
DROP FUNCTION IF EXISTS validar_stock_venta();
DROP FUNCTION IF EXISTS actualizar_fecha_modificacion();

-- ===== ELIMINAR TABLAS =====
DROP TABLE IF EXISTS detalle_inventarios;
DROP TABLE IF EXISTS inventarios;
DROP TABLE IF EXISTS detalle_ventas;
DROP TABLE IF EXISTS ventas;
DROP TABLE IF EXISTS productos;
DROP TABLE IF EXISTS personas;

-- ===== ELIMINAR TIPOS ENUM =====
DROP TYPE IF EXISTS tipo_movimiento;
DROP TYPE IF EXISTS tipo_perfil;

-- ===== ELIMINAR EXTENSIONES =====
DROP EXTENSION IF EXISTS "uuid-ossp";
