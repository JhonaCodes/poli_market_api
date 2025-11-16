use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{Utc, NaiveDateTime};
use uuid::Uuid;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::ventas::model::{
    CrearVentaRequest, VentaCreadaResponse, VentaResponse, DetalleVentaResponse,
    NuevaVenta, NuevoDetalleVenta,
};
use crate::modules::ventas::repository::VentaRepository;
use crate::modules::personas::repository::PersonaRepository;
use crate::modules::productos::repository::ProductoRepository;
use crate::modules::inventarios::repository::InventarioRepository;

pub struct VentaService {
    venta_repo: VentaRepository,
    persona_repo: PersonaRepository,
    producto_repo: ProductoRepository,
    inventario_repo: InventarioRepository,
}

impl VentaService {
    pub fn new(
        venta_repo: VentaRepository,
        persona_repo: PersonaRepository,
        producto_repo: ProductoRepository,
        inventario_repo: InventarioRepository,
    ) -> Self {
        VentaService {
            venta_repo,
            persona_repo,
            producto_repo,
            inventario_repo,
        }
    }

    /// RF1: Procesar una venta con descuento automático de inventario
    pub fn procesar_venta(&self, request: CrearVentaRequest) -> ApiResult<VentaCreadaResponse> {
        // 1. Validar que el cliente existe y está activo
        let id_cliente = Uuid::parse_str(&request.id_cliente)
            .map_err(|_| ApiError::InvalidInput("ID de cliente inválido".to_string()))?;

        let cliente = self.persona_repo.buscar_por_id(id_cliente)?;
        if !cliente.activo {
            return Err(ApiError::InactiveClient);
        }

        // 2. Validar que hay detalles
        if request.detalles.is_empty() {
            return Err(ApiError::InvalidInput("La venta debe tener al menos un detalle".to_string()));
        }

        // 3. Validar stock y calcular total
        let mut total = BigDecimal::from(0);
        let mut detalles_validados = Vec::new();

        for detalle_req in &request.detalles {
            let id_producto = Uuid::parse_str(&detalle_req.id_producto)
                .map_err(|_| ApiError::InvalidInput("ID de producto inválido".to_string()))?;

            // Verificar que el producto existe
            let producto = self.producto_repo.buscar_por_id(id_producto)?;

            // Validar cantidad positiva
            if detalle_req.cantidad <= 0 {
                return Err(ApiError::InvalidInput("La cantidad debe ser mayor a 0".to_string()));
            }

            // Validar stock suficiente
            let stock_actual = self.inventario_repo.obtener_stock(id_producto)?;
            if stock_actual < detalle_req.cantidad {
                return Err(ApiError::BusinessRuleViolation(
                    format!("Stock insuficiente para el producto '{}'. Disponible: {}, Requerido: {}",
                            producto.nombre, stock_actual, detalle_req.cantidad)
                ));
            }

            // Calcular subtotal
            let subtotal = &producto.precio_unitario * BigDecimal::from(detalle_req.cantidad);
            total += &subtotal;

            detalles_validados.push((id_producto, detalle_req.cantidad, subtotal));
        }

        // 4. Crear la venta
        let venta_id = Uuid::new_v4();
        let fecha_actual = Utc::now().naive_utc();

        let nueva_venta = NuevaVenta {
            id: venta_id,
            id_persona: id_cliente,
            fecha: fecha_actual,
            monto: total.clone(),
            sucursal: request.sucursal.clone(),
        };

        let nuevos_detalles: Vec<NuevoDetalleVenta> = detalles_validados
            .iter()
            .map(|(id_producto, cantidad, monto)| NuevoDetalleVenta {
                id: Uuid::new_v4(),
                id_venta: venta_id,
                id_producto: *id_producto,
                cantidad: *cantidad,
                monto: monto.clone(),
            })
            .collect();

        // 5. Guardar venta en transacción (esto también descuenta el inventario automáticamente
        // gracias a los triggers de la base de datos)
        self.venta_repo.guardar_con_detalles(nueva_venta, nuevos_detalles)?;

        Ok(VentaCreadaResponse {
            id: venta_id.to_string(),
            mensaje: format!("Venta creada exitosamente. Total: ${:.2}", total.to_f64().unwrap_or(0.0)),
        })
    }

    /// RF2: Obtener ventas con filtros opcionales
    pub fn obtener_ventas(
        &self,
        id_cliente: Option<String>,
        sucursal: Option<String>,
        fecha_desde: Option<String>,
        fecha_hasta: Option<String>,
    ) -> ApiResult<Vec<VentaResponse>> {
        let id_cliente_uuid = match id_cliente {
            Some(id_str) => Some(Uuid::parse_str(&id_str)
                .map_err(|_| ApiError::InvalidInput("ID de cliente inválido".to_string()))?),
            None => None,
        };

        let fecha_desde_naive = match fecha_desde {
            Some(fecha_str) => Some(NaiveDateTime::parse_from_str(&fecha_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| ApiError::InvalidInput("Formato de fecha inválido".to_string()))?),
            None => None,
        };

        let fecha_hasta_naive = match fecha_hasta {
            Some(fecha_str) => Some(NaiveDateTime::parse_from_str(&fecha_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| ApiError::InvalidInput("Formato de fecha inválido".to_string()))?),
            None => None,
        };

        let ventas = self.venta_repo.listar(
            id_cliente_uuid,
            sucursal,
            fecha_desde_naive,
            fecha_hasta_naive,
        )?;

        let mut respuestas = Vec::new();
        for venta in ventas {
            let detalles = self.venta_repo.obtener_detalles(venta.id)?;

            let mut detalles_response = Vec::new();
            for detalle in detalles {
                let producto = self.producto_repo.buscar_por_id(detalle.id_producto)?;
                detalles_response.push(DetalleVentaResponse {
                    id_producto: detalle.id_producto.to_string(),
                    nombre_producto: producto.nombre,
                    cantidad: detalle.cantidad,
                    precio_unitario: producto.precio_unitario.to_f64().unwrap_or(0.0),
                    subtotal: detalle.monto.to_f64().unwrap_or(0.0),
                });
            }

            respuestas.push(VentaResponse {
                id: venta.id.to_string(),
                id_cliente: venta.id_persona.to_string(),
                fecha: venta.fecha.format("%Y-%m-%d %H:%M:%S").to_string(),
                total: venta.monto.to_f64().unwrap_or(0.0),
                sucursal: venta.sucursal,
                detalles: detalles_response,
            });
        }

        Ok(respuestas)
    }

    /// Obtener una venta específica por ID
    pub fn obtener_venta_por_id(&self, id_str: &str) -> ApiResult<VentaResponse> {
        let id = Uuid::parse_str(id_str)
            .map_err(|_| ApiError::InvalidInput("ID de venta inválido".to_string()))?;

        let (venta, detalles) = self.venta_repo.buscar_por_id(id)?;

        let mut detalles_response = Vec::new();
        for detalle in detalles {
            let producto = self.producto_repo.buscar_por_id(detalle.id_producto)?;
            detalles_response.push(DetalleVentaResponse {
                id_producto: detalle.id_producto.to_string(),
                nombre_producto: producto.nombre,
                cantidad: detalle.cantidad,
                precio_unitario: producto.precio_unitario.to_f64().unwrap_or(0.0),
                subtotal: detalle.monto.to_f64().unwrap_or(0.0),
            });
        }

        Ok(VentaResponse {
            id: venta.id.to_string(),
            id_cliente: venta.id_persona.to_string(),
            fecha: venta.fecha.format("%Y-%m-%d %H:%M:%S").to_string(),
            total: venta.monto.to_f64().unwrap_or(0.0),
            sucursal: venta.sucursal,
            detalles: detalles_response,
        })
    }
}
