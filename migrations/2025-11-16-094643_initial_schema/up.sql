-- ===== EXTENSIONES =====
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ===== TIPOS ENUM =====

CREATE TYPE tipo_perfil AS ENUM ('VENDEDOR', 'CLIENTE', 'PROVEEDOR');
CREATE TYPE tipo_movimiento AS ENUM ('ENTRADA', 'SALIDA', 'AJUSTE');

-- ===== TABLA: personas =====
CREATE TABLE personas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nombre VARCHAR(255) NOT NULL,
    documento VARCHAR(50) UNIQUE NOT NULL,
    perfil tipo_perfil NOT NULL,
    email VARCHAR(255),
    telefono VARCHAR(20),
    fecha_creacion TIMESTAMP NOT NULL DEFAULT NOW(),
    fecha_actualizacion TIMESTAMP NOT NULL DEFAULT NOW(),
    activo BOOLEAN NOT NULL DEFAULT TRUE,

    CONSTRAINT chk_documento_valido CHECK (LENGTH(documento) >= 5),
    CONSTRAINT chk_nombre_valido CHECK (LENGTH(nombre) >= 2)
);

CREATE INDEX idx_personas_documento ON personas(documento);
CREATE INDEX idx_personas_perfil ON personas(perfil);
CREATE INDEX idx_personas_activo ON personas(activo);

-- ===== TABLA: productos =====
CREATE TABLE productos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nombre VARCHAR(255) NOT NULL,
    cantidad INT NOT NULL DEFAULT 0,
    unidad_venta VARCHAR(50) NOT NULL,
    precio_unitario NUMERIC(12, 2) NOT NULL,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT NOW(),
    fecha_actualizacion TIMESTAMP NOT NULL DEFAULT NOW(),
    activo BOOLEAN NOT NULL DEFAULT TRUE,

    CONSTRAINT chk_precio_positivo CHECK (precio_unitario > 0),
    CONSTRAINT chk_cantidad_positiva CHECK (cantidad >= 0),
    CONSTRAINT chk_nombre_producto CHECK (LENGTH(nombre) >= 2)
);

CREATE INDEX idx_productos_nombre ON productos(nombre);
CREATE INDEX idx_productos_activo ON productos(activo);

-- ===== TABLA: ventas =====
CREATE TABLE ventas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_persona UUID NOT NULL REFERENCES personas(id),
    fecha TIMESTAMP NOT NULL DEFAULT NOW(),
    monto NUMERIC(12, 2) NOT NULL,
    sucursal VARCHAR(100),
    fecha_creacion TIMESTAMP NOT NULL DEFAULT NOW(),
    fecha_actualizacion TIMESTAMP NOT NULL DEFAULT NOW(),
    activo BOOLEAN NOT NULL DEFAULT TRUE,

    CONSTRAINT chk_monto_positivo CHECK (monto >= 0)
);

CREATE INDEX idx_ventas_persona ON ventas(id_persona);
CREATE INDEX idx_ventas_fecha ON ventas(fecha);
CREATE INDEX idx_ventas_activo ON ventas(activo);

-- ===== TABLA: detalle_ventas =====
CREATE TABLE detalle_ventas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_venta UUID NOT NULL REFERENCES ventas(id) ON DELETE CASCADE,
    id_producto UUID NOT NULL REFERENCES productos(id),
    cantidad INT NOT NULL,
    monto NUMERIC(12, 2) NOT NULL,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT NOW(),
    fecha_actualizacion TIMESTAMP NOT NULL DEFAULT NOW(),
    activo BOOLEAN NOT NULL DEFAULT TRUE,

    CONSTRAINT chk_detalle_cantidad_positiva CHECK (cantidad > 0),
    CONSTRAINT chk_detalle_monto_positivo CHECK (monto >= 0)
);

CREATE INDEX idx_detalle_ventas_venta ON detalle_ventas(id_venta);
CREATE INDEX idx_detalle_ventas_producto ON detalle_ventas(id_producto);

-- ===== TABLA: inventarios =====
CREATE TABLE inventarios (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_producto UUID NOT NULL UNIQUE REFERENCES productos(id),
    id_persona UUID NOT NULL REFERENCES personas(id),
    cantidad_disponible INT NOT NULL DEFAULT 0,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT NOW(),
    fecha_actualizacion TIMESTAMP NOT NULL DEFAULT NOW(),
    activo BOOLEAN NOT NULL DEFAULT TRUE,

    CONSTRAINT chk_inventario_cantidad CHECK (cantidad_disponible >= 0)
);

CREATE INDEX idx_inventarios_producto ON inventarios(id_producto);
CREATE INDEX idx_inventarios_persona ON inventarios(id_persona);

-- ===== TABLA: detalle_inventarios =====
CREATE TABLE detalle_inventarios (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    id_producto UUID NOT NULL REFERENCES productos(id),
    tipo_movimiento tipo_movimiento NOT NULL,
    fecha TIMESTAMP NOT NULL DEFAULT NOW(),
    id_persona UUID NOT NULL REFERENCES personas(id),
    cantidad INT NOT NULL,
    observaciones TEXT,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT NOW(),
    fecha_actualizacion TIMESTAMP NOT NULL DEFAULT NOW(),
    activo BOOLEAN NOT NULL DEFAULT TRUE,

    CONSTRAINT chk_cantidad_movimiento CHECK (cantidad != 0)
);

CREATE INDEX idx_detalle_inventarios_producto ON detalle_inventarios(id_producto);
CREATE INDEX idx_detalle_inventarios_fecha ON detalle_inventarios(fecha);
CREATE INDEX idx_detalle_inventarios_tipo ON detalle_inventarios(tipo_movimiento);

-- ===== FUNCIÓN: Actualizar fecha_actualizacion =====
CREATE OR REPLACE FUNCTION actualizar_fecha_modificacion()
RETURNS TRIGGER AS $$
BEGIN
    NEW.fecha_actualizacion = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ===== TRIGGERS para actualización automática =====
CREATE TRIGGER trg_personas_actualizacion
    BEFORE UPDATE ON personas
    FOR EACH ROW
    EXECUTE FUNCTION actualizar_fecha_modificacion();

CREATE TRIGGER trg_productos_actualizacion
    BEFORE UPDATE ON productos
    FOR EACH ROW
    EXECUTE FUNCTION actualizar_fecha_modificacion();

CREATE TRIGGER trg_ventas_actualizacion
    BEFORE UPDATE ON ventas
    FOR EACH ROW
    EXECUTE FUNCTION actualizar_fecha_modificacion();

CREATE TRIGGER trg_inventarios_actualizacion
    BEFORE UPDATE ON inventarios
    FOR EACH ROW
    EXECUTE FUNCTION actualizar_fecha_modificacion();

-- ===== FUNCIÓN: Validar stock antes de venta =====
CREATE OR REPLACE FUNCTION validar_stock_venta()
RETURNS TRIGGER AS $$
DECLARE
    stock_actual INT;
BEGIN
    SELECT cantidad_disponible INTO stock_actual
    FROM inventarios
    WHERE id_producto = NEW.id_producto;

    IF stock_actual IS NULL OR stock_actual < NEW.cantidad THEN
        RAISE EXCEPTION 'Stock insuficiente para el producto %', NEW.id_producto;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_validar_stock_detalle_venta
    BEFORE INSERT ON detalle_ventas
    FOR EACH ROW
    EXECUTE FUNCTION validar_stock_venta();

-- ===== FUNCIÓN: Actualizar inventario después de venta =====
CREATE OR REPLACE FUNCTION actualizar_inventario_venta()
RETURNS TRIGGER AS $$
BEGIN
    -- Reducir stock
    UPDATE inventarios
    SET cantidad_disponible = cantidad_disponible - NEW.cantidad
    WHERE id_producto = NEW.id_producto;

    -- Registrar movimiento
    INSERT INTO detalle_inventarios (
        id_producto,
        tipo_movimiento,
        fecha,
        id_persona,
        cantidad,
        observaciones
    )
    SELECT
        NEW.id_producto,
        'SALIDA'::tipo_movimiento,
        NOW(),
        v.id_persona,
        -NEW.cantidad,
        'Venta ID: ' || NEW.id_venta
    FROM ventas v
    WHERE v.id = NEW.id_venta;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_actualizar_inventario_venta
    AFTER INSERT ON detalle_ventas
    FOR EACH ROW
    EXECUTE FUNCTION actualizar_inventario_venta();

-- ===== VISTAS =====

-- Vista: Ventas con detalles
CREATE VIEW vista_ventas_completas AS
SELECT
    v.id AS venta_id,
    v.fecha,
    v.monto AS total,
    p.nombre AS cliente_nombre,
    p.documento AS cliente_documento,
    json_agg(
        json_build_object(
            'producto_id', pr.id,
            'producto_nombre', pr.nombre,
            'cantidad', dv.cantidad,
            'precio_unitario', pr.precio_unitario,
            'subtotal', dv.monto
        )
    ) AS detalles
FROM ventas v
JOIN personas p ON v.id_persona = p.id
JOIN detalle_ventas dv ON v.id = dv.id_venta
JOIN productos pr ON dv.id_producto = pr.id
WHERE v.activo = TRUE
GROUP BY v.id, v.fecha, v.monto, p.nombre, p.documento;

-- Vista: Stock actual por producto
CREATE VIEW vista_stock_productos AS
SELECT
    p.id AS producto_id,
    p.nombre AS producto_nombre,
    p.precio_unitario,
    COALESCE(i.cantidad_disponible, 0) AS stock_actual,
    CASE
        WHEN COALESCE(i.cantidad_disponible, 0) = 0 THEN 'SIN_STOCK'
        WHEN COALESCE(i.cantidad_disponible, 0) < 10 THEN 'STOCK_BAJO'
        ELSE 'STOCK_OK'
    END AS estado_stock
FROM productos p
LEFT JOIN inventarios i ON p.id = i.id_producto
WHERE p.activo = TRUE;

-- ===== DATOS DE PRUEBA =====

-- Personas
INSERT INTO personas (nombre, documento, perfil, email) VALUES
('Juan Pérez', '12345678', 'CLIENTE', 'juan@email.com'),
('María González', '87654321', 'VENDEDOR', 'maria@email.com'),
('Proveedor ABC', '11111111', 'PROVEEDOR', 'proveedor@abc.com');

-- Productos
INSERT INTO productos (nombre, cantidad, unidad_venta, precio_unitario) VALUES
('Laptop Dell', 10, 'Unidad', 1200.00),
('Mouse Logitech', 50, 'Unidad', 25.00),
('Teclado Mecánico', 30, 'Unidad', 80.00);

-- Inicializar inventarios
INSERT INTO inventarios (id_producto, id_persona, cantidad_disponible)
SELECT
    p.id,
    (SELECT id FROM personas WHERE perfil = 'VENDEDOR' LIMIT 1),
    p.cantidad
FROM productos p;

-- ===== COMENTARIOS =====
COMMENT ON VIEW vista_ventas_completas IS 'Ventas con todos sus detalles';
COMMENT ON VIEW vista_stock_productos IS 'Estado actual del inventario';
