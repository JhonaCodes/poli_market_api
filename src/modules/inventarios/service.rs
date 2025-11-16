use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::common::types::TipoMovimiento;
use crate::modules::inventarios::model::{DisponibilidadResponse, MovimientoRequest, MovimientoRegistradoResponse};
use crate::modules::inventarios::repository::InventarioRepository;
use crate::modules::productos::repository::ProductoRepository;
use crate::modules::personas::repository::PersonaRepository;

pub struct InventarioService {
    inventario_repo: InventarioRepository,
    producto_repo: ProductoRepository,
    persona_repo: PersonaRepository,
}

impl InventarioService {
    pub fn new(
        inventario_repo: InventarioRepository,
        producto_repo: ProductoRepository,
        persona_repo: PersonaRepository,
    ) -> Self {
        InventarioService {
            inventario_repo,
            producto_repo,
            persona_repo,
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

    /// Registrar un movimiento de inventario (ENTRADA, SALIDA, AJUSTE)
    pub fn registrar_movimiento(&self, request: MovimientoRequest) -> ApiResult<MovimientoRegistradoResponse> {
        // Validar ID de producto
        let id_producto = Uuid::parse_str(&request.id_producto)
            .map_err(|_| ApiError::InvalidInput("ID de producto inválido".to_string()))?;

        // Validar que el producto existe
        let producto = self.producto_repo.buscar_por_id(id_producto)?;

        // Validar ID de persona
        let id_persona = Uuid::parse_str(&request.id_persona)
            .map_err(|_| ApiError::InvalidInput("ID de persona inválido".to_string()))?;

        // Validar que la persona existe y está activa
        let persona = self.persona_repo.buscar_por_id(id_persona)?;
        if !persona.activo {
            return Err(ApiError::InactiveClient);
        }

        // Validar cantidad positiva
        if request.cantidad <= 0 {
            return Err(ApiError::InvalidInput("La cantidad debe ser mayor a 0".to_string()));
        }

        // Parsear y validar tipo de movimiento
        let tipo_movimiento = match request.tipo_movimiento.to_uppercase().as_str() {
            "ENTRADA" => TipoMovimiento::Entrada,
            "SALIDA" => TipoMovimiento::Salida,
            "AJUSTE" => TipoMovimiento::Ajuste,
            _ => return Err(ApiError::InvalidInput(
                "Tipo de movimiento inválido. Valores permitidos: ENTRADA, SALIDA, AJUSTE".to_string()
            )),
        };

        // Registrar el movimiento con actualización de stock
        let id = self.inventario_repo.registrar_movimiento_con_actualizacion(
            id_producto,
            tipo_movimiento,
            id_persona,
            request.cantidad,
            request.observaciones,
        )?;

        let mensaje = match tipo_movimiento {
            TipoMovimiento::Entrada => format!(
                "Entrada registrada: +{} unidades de '{}'. Stock actualizado.",
                request.cantidad, producto.nombre
            ),
            TipoMovimiento::Salida => format!(
                "Salida registrada: -{} unidades de '{}'. Stock actualizado.",
                request.cantidad, producto.nombre
            ),
            TipoMovimiento::Ajuste => format!(
                "Ajuste registrado: {} unidades de '{}'. Stock actualizado.",
                request.cantidad, producto.nombre
            ),
        };

        Ok(MovimientoRegistradoResponse {
            id: id.to_string(),
            mensaje,
        })
    }
}
