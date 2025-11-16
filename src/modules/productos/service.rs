use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::productos::model::ProductoResponse;
use crate::modules::productos::repository::ProductoRepository;
use crate::modules::inventarios::repository::InventarioRepository;
use bigdecimal::ToPrimitive;

pub struct ProductoService {
    producto_repo: ProductoRepository,
    inventario_repo: InventarioRepository,
}

impl ProductoService {
    pub fn new(producto_repo: ProductoRepository, inventario_repo: InventarioRepository) -> Self {
        ProductoService {
            producto_repo,
            inventario_repo,
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
}
