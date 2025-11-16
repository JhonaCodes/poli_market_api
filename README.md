# PoliMarket API

API REST para el sistema de gestión PoliMarket, implementado con Rust, Actix-web y Diesel ORM.

## Arquitectura

El proyecto sigue una arquitectura modular en 4 capas siguiendo principios de arquitectura hexagonal:

```
src/
├── modules/           # Módulos de negocio
│   ├── common/       # Tipos y errores compartidos
│   ├── personas/     # RF4: Gestión de personas
│   ├── productos/    # RF3: Gestión de productos
│   ├── inventarios/  # RF5: Gestión de inventario
│   └── ventas/       # RF1, RF2: Gestión de ventas
├── config/           # Configuración de la aplicación
├── state/            # Estado de la aplicación (servicios)
└── schema.rs         # Esquema de base de datos (Diesel)
```

### Estructura de cada módulo:

```
módulo/
├── model.rs          # Entidades de dominio y DTOs
├── repository.rs     # Acceso a datos (infraestructura)
├── service.rs        # Lógica de negocio (aplicación)
└── handler.rs        # Controladores HTTP (presentación)
```

## Requisitos Funcionales Implementados

### RF1: Procesar Venta
- **Endpoint:** `POST /api/ventas`
- **Descripción:** Crea una venta con descuento automático de inventario
- **Validaciones:**
  - Cliente existe y está activo
  - Stock suficiente para cada producto
  - Cantidades válidas

### RF2: Consultar Ventas
- **Endpoints:**
  - `GET /api/ventas` - Listar ventas con filtros
  - `GET /api/ventas/:id` - Obtener venta específica
- **Filtros disponibles:**
  - `id_cliente`
  - `sucursal`
  - `fecha_desde`
  - `fecha_hasta`

### RF3: Obtener Producto
- **Endpoints:**
  - `GET /api/productos` - Listar productos
  - `GET /api/productos/:id` - Obtener producto con stock actual

### RF4: Obtener Persona
- **Endpoints:**
  - `GET /api/personas` - Listar personas
  - `GET /api/personas/:id` - Obtener persona específica
- **Filtros disponibles:**
  - `tipo` (VENDEDOR, CLIENTE, PROVEEDOR)

### RF5: Consultar Disponibilidad de Inventario
- **Endpoint:** `GET /api/inventario/disponibilidad/:id`
- **Descripción:** Consulta el stock actual de un producto

## Tecnologías

- **Rust 1.90+**
- **Actix-web 4.11** - Framework web
- **Diesel 2.3** - ORM para PostgreSQL
- **PostgreSQL 15** - Base de datos
- **Docker & Docker Compose** - Contenedorización

## Requisitos Previos

- Rust 1.90 o superior
- PostgreSQL 15 o superior
- Docker y Docker Compose (para despliegue con contenedores)

## Instalación y Configuración

### Opción 1: Desarrollo Local

#### 1. Clonar el repositorio

```bash
cd poli_market_api
```

#### 2. Configurar variables de entorno

```bash
cp .env.example .env
# Editar .env con tus credenciales de PostgreSQL
```

#### 3. Crear base de datos

```bash
createdb polimarket
psql -d polimarket -f ../polimarket_schema_postgresql.sql
```

#### 4. Instalar Diesel CLI (opcional)

```bash
cargo install diesel_cli --no-default-features --features postgres
```

#### 5. Compilar y ejecutar

```bash
cargo build --release
cargo run
```

El servidor estará disponible en `http://localhost:8080`

### Opción 2: Docker Compose (Recomendado)

#### 1. Iniciar servicios

```bash
docker-compose up -d
```

Esto iniciará:
- PostgreSQL en el puerto 5432
- API en el puerto 8080

#### 2. Ver logs

```bash
docker-compose logs -f api
```

#### 3. Detener servicios

```bash
docker-compose down
```

## Endpoints API

### Health Check

```bash
GET /api/health
```

### Personas

```bash
# Listar personas
GET /api/personas?tipo=CLIENTE

# Obtener persona por ID
GET /api/personas/{id}
```

### Productos

```bash
# Listar productos
GET /api/productos

# Obtener producto por ID
GET /api/productos/{id}
```

### Inventario

```bash
# Consultar disponibilidad
GET /api/inventario/disponibilidad/{id_producto}
```

### Ventas

```bash
# Crear venta
POST /api/ventas
Content-Type: application/json

{
  "id_cliente": "uuid-del-cliente",
  "sucursal": "Sucursal Centro",
  "detalles": [
    {
      "id_producto": "uuid-del-producto",
      "cantidad": 2
    }
  ]
}

# Listar ventas
GET /api/ventas?id_cliente=uuid&sucursal=Centro

# Obtener venta específica
GET /api/ventas/{id}
```

## Ejemplos de Uso

### Crear una venta

```bash
curl -X POST http://localhost:8080/api/ventas \
  -H "Content-Type: application/json" \
  -d '{
    "id_cliente": "550e8400-e29b-41d4-a716-446655440000",
    "sucursal": "Sucursal Centro",
    "detalles": [
      {
        "id_producto": "660e8400-e29b-41d4-a716-446655440000",
        "cantidad": 2
      }
    ]
  }'
```

### Consultar disponibilidad de inventario

```bash
curl http://localhost:8080/api/inventario/disponibilidad/660e8400-e29b-41d4-a716-446655440000
```

### Listar productos

```bash
curl http://localhost:8080/api/productos
```

## Arquitectura de Base de Datos

La base de datos utiliza:
- **Triggers automáticos** para actualizar inventario en ventas
- **Constraints** para validar integridad de datos
- **Índices** para optimizar consultas
- **Soft delete** con campo `activo`
- **Auditoría** con `fecha_creacion` y `fecha_actualizacion`

Ver el esquema completo en: `../polimarket_schema_postgresql.sql`

## Manejo de Errores

La API retorna errores en formato JSON:

```json
{
  "error": "Stock insuficiente para el producto",
  "code": "INSUFFICIENT_STOCK"
}
```

Códigos de error comunes:
- `NOT_FOUND` - Recurso no encontrado (404)
- `INVALID_INPUT` - Entrada inválida (400)
- `BUSINESS_RULE_VIOLATION` - Violación de regla de negocio (400)
- `INSUFFICIENT_STOCK` - Stock insuficiente (400)
- `INACTIVE_CLIENT` - Cliente inactivo (400)
- `INTERNAL_ERROR` - Error interno del servidor (500)

## Logging

El nivel de log se configura con la variable de entorno `RUST_LOG`:

```bash
RUST_LOG=debug cargo run
```

Niveles disponibles: `error`, `warn`, `info`, `debug`, `trace`

## Testing

```bash
# Ejecutar tests
cargo test

# Con output detallado
cargo test -- --nocapture
```

## Compilación Optimizada

```bash
# Build de producción
cargo build --release

# El binario estará en target/release/poli_market_api
```

## Estructura de Código

### Capa de Dominio (Models)
Define las entidades de negocio y DTOs sin lógica de infraestructura.

### Capa de Infraestructura (Repositories)
Implementa el acceso a datos usando Diesel ORM con transacciones.

### Capa de Aplicación (Services)
Contiene toda la lógica de negocio y orquesta los repositorios.

### Capa de Presentación (Handlers)
Maneja las peticiones HTTP y delega al servicio correspondiente.

## Principios SOLID Aplicados

- **Single Responsibility:** Cada módulo tiene una responsabilidad única
- **Open/Closed:** Extensible mediante nuevos módulos
- **Liskov Substitution:** Repositorios intercambiables
- **Interface Segregation:** Servicios con interfaces específicas
- **Dependency Inversion:** Dependencias mediante traits/abstracciones

## Seguridad

- Variables de entorno para credenciales sensibles
- Pool de conexiones con límites
- Validación de entrada en todos los endpoints
- CORS configurado
- Usuario no-root en contenedor Docker

## Licencia

Proyecto académico - Politécnico Grancolombiano

## Autor

Maestría en Arquitectura de Software
Noviembre 2025
