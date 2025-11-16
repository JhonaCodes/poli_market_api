use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use crate::modules::personas::repository::PersonaRepository;
use crate::modules::personas::service::PersonaService;
use crate::modules::productos::repository::ProductoRepository;
use crate::modules::productos::service::ProductoService;
use crate::modules::inventarios::repository::InventarioRepository;
use crate::modules::inventarios::service::InventarioService;
use crate::modules::ventas::repository::VentaRepository;
use crate::modules::ventas::service::VentaService;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pub persona_service: PersonaService,
    pub producto_service: ProductoService,
    pub inventario_service: InventarioService,
    pub venta_service: VentaService,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        // Create repositories (unused for now, but kept for future optimization)
        let _persona_repo = PersonaRepository::new(pool.clone());
        let _producto_repo = ProductoRepository::new(pool.clone());
        let _inventario_repo = InventarioRepository::new(pool.clone());
        let _venta_repo = VentaRepository::new(pool.clone());

        // Create services with their dependencies
        let persona_service = PersonaService::new(
            PersonaRepository::new(pool.clone())
        );

        let inventario_service = InventarioService::new(
            InventarioRepository::new(pool.clone()),
            ProductoRepository::new(pool.clone()),
        );

        let producto_service = ProductoService::new(
            ProductoRepository::new(pool.clone()),
            InventarioRepository::new(pool.clone()),
        );

        let venta_service = VentaService::new(
            VentaRepository::new(pool.clone()),
            PersonaRepository::new(pool.clone()),
            ProductoRepository::new(pool.clone()),
            InventarioRepository::new(pool.clone()),
        );

        AppState {
            persona_service,
            producto_service,
            inventario_service,
            venta_service,
        }
    }
}
