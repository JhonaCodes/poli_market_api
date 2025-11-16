use uuid::Uuid;
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::Connection;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::productos::model::{ProductoResponse, CrearProductoRequest, ProductoCreadoResponse, NuevoProducto};
use crate::modules::productos::repository::ProductoRepository;
use crate::modules::inventarios::repository::InventarioRepository;
use crate::modules::personas::repository::PersonaRepository;

pub struct ProductoService {
    producto_repo: ProductoRepository,
    inventario_repo: InventarioRepository,
    persona_repo: PersonaRepository,
}

impl ProductoService {
    pub fn new(
        producto_repo: ProductoRepository,
        inventario_repo: InventarioRepository,
        persona_repo: PersonaRepository,
    ) -> Self {
        ProductoService {
            producto_repo,
            inventario_repo,
            persona_repo,
        }
    }

    /// RF3: Obtener producto por ID con su stock actual
    pub fn obtener_producto(&self, id_str: &str) -> ApiResult<ProductoResponse> {
        let id = Uuid::parse_str(id_str)
            .map_err(|_| ApiError::InvalidInput("ID de producto inválido".to_string()))?;

        let producto = self.producto_repo.buscar_por_id(id)?;
        let stock_actual = self.inventario_repo.obtener_stock(id)?;

        Ok(ProductoResponse {
            id: producto.id.to_string(),
            nombre: producto.nombre,
            precio_unitario: producto.precio_unitario.to_f64().unwrap_or(0.0),
            unidad_venta: producto.unidad_venta,
            stock_actual,
        })
    }

    /// Listar todos los productos activos con su stock
    pub fn listar_productos(&self) -> ApiResult<Vec<ProductoResponse>> {
        let productos = self.producto_repo.listar()?;

        let mut respuestas = Vec::new();
        for producto in productos {
            let stock_actual = self.inventario_repo.obtener_stock(producto.id)?;
            respuestas.push(ProductoResponse {
                id: producto.id.to_string(),
                nombre: producto.nombre,
                precio_unitario: producto.precio_unitario.to_f64().unwrap_or(0.0),
                unidad_venta: producto.unidad_venta,
                stock_actual,
            });
        }

        Ok(respuestas)
    }

    /// Crear un nuevo producto con su inventario inicial
    pub fn crear_producto(&self, request: CrearProductoRequest) -> ApiResult<ProductoCreadoResponse> {
        // Validar campos requeridos
        if request.nombre.trim().is_empty() {
            return Err(ApiError::InvalidInput("El nombre del producto es requerido".to_string()));
        }

        if request.unidad_venta.trim().is_empty() {
            return Err(ApiError::InvalidInput("La unidad de venta es requerida".to_string()));
        }

        // Validar que la cantidad sea no negativa
        if request.cantidad < 0 {
            return Err(ApiError::InvalidInput("La cantidad no puede ser negativa".to_string()));
        }

        // Validar que el precio sea positivo
        if request.precio_unitario <= 0.0 {
            return Err(ApiError::InvalidInput("El precio unitario debe ser mayor a 0".to_string()));
        }

        // Para el inventario inicial, necesitamos una persona responsable
        // Por ahora, usaremos el primer vendedor activo que encontremos
        // En un sistema real, esto vendría del contexto del usuario autenticado
        let personas = self.persona_repo.listar(Some(crate::modules::common::types::TipoPerfil::Vendedor))?;
        let id_persona = personas.first()
            .ok_or_else(|| ApiError::BusinessRuleViolation(
                "No hay vendedores registrados en el sistema. Debe crear al menos un vendedor primero".to_string()
            ))?
            .id;

        // Crear el producto con transacción (producto + inventario inicial)
        let mut conn = self.producto_repo.pool.get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let id_producto = conn.transaction::<Uuid, ApiError, _>(|conn| {
            let id_producto = Uuid::new_v4();

            // Crear el producto
            let nuevo_producto = NuevoProducto {
                nombre: request.nombre.trim().to_string(),
                cantidad: request.cantidad,
                unidad_venta: request.unidad_venta.trim().to_string(),
                precio_unitario: BigDecimal::try_from(request.precio_unitario)
                    .map_err(|e| ApiError::InvalidInput(format!("Precio inválido: {}", e)))?,
            };

            self.producto_repo.crear_con_conexion(conn, id_producto, nuevo_producto)?;

            // Crear el inventario inicial
            self.inventario_repo.crear_inventario_inicial(
                conn,
                id_producto,
                id_persona,
                request.cantidad
            )?;

            Ok(id_producto)
        })?;

        Ok(ProductoCreadoResponse {
            id: id_producto.to_string(),
            mensaje: format!(
                "Producto creado exitosamente con inventario inicial de {} unidades",
                request.cantidad
            ),
        })
    }
}
