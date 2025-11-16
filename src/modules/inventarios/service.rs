use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::inventarios::model::DisponibilidadResponse;
use crate::modules::inventarios::repository::InventarioRepository;
use crate::modules::productos::repository::ProductoRepository;

pub struct InventarioService {
    inventario_repo: InventarioRepository,
    producto_repo: ProductoRepository,
}

impl InventarioService {
    pub fn new(inventario_repo: InventarioRepository, producto_repo: ProductoRepository) -> Self {
        InventarioService {
            inventario_repo,
            producto_repo,
        }
    }

    /// RF5: Consultar disponibilidad de inventario
    pub fn obtener_disponibilidad(&self, id_producto_str: &str) -> ApiResult<DisponibilidadResponse> {
        let id_producto = Uuid::parse_str(id_producto_str)
            .map_err(|_| ApiError::InvalidInput("ID de producto inválido".to_string()))?;

        // Verificar que el producto existe
        self.producto_repo.verificar_existe_y_activo(id_producto)?;

        let cantidad_disponible = self.inventario_repo.obtener_stock(id_producto)?;

        Ok(DisponibilidadResponse {
            id_producto: id_producto.to_string(),
            cantidad_disponible,
        })
    }

    /// Validar que hay suficiente stock para una cantidad requerida
    pub fn validar_stock(&self, id_producto: Uuid, cantidad_requerida: i32) -> ApiResult<()> {
        let stock_disponible = self.inventario_repo.obtener_stock(id_producto)?;

        if stock_disponible < cantidad_requerida {
            return Err(ApiError::InsufficientStock);
        }

        Ok(())
    }
}
