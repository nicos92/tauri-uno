use std::collections::BTreeMap;
use std::sync::Arc;

use chrono::{Datelike, Local, NaiveDate, TimeZone};

use crate::domain::entities::{Cierre, CierreTipo, CierreWithTipos};
use crate::domain::repositories::CierreRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteCierreRepository;

pub struct CierreService {
    repository: Arc<SqliteCierreRepository>,
}

impl CierreService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteCierreRepository::new()),
        }
    }

    pub fn crear_cierre(&self, fecha: &str) -> Result<CierreWithTipos, AppError> {
        let day = NaiveDate::parse_from_str(fecha, "%Y-%m-%d")
            .map_err(|e| AppError::Internal(format!("Fecha inválida: {}", e)))?;

        if self.repository.find_by_fecha(fecha)?.is_some() {
            return Err(AppError::CierreYaExiste);
        }

        let start = local_to_utc(&day.and_hms_opt(0, 0, 0).unwrap());
        let end = local_to_utc(&(day + chrono::Duration::days(1)).and_hms_opt(0, 0, 0).unwrap());

        let conn = crate::infrastructure::database::DB
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let mut ventas_by_tipo: BTreeMap<i64, (String, f64)> = BTreeMap::new();
        {
            let mut stmt = conn.prepare(
                "SELECT v.id_tipo_venta, COALESCE(t.nombre, 'Efectivo'), v.total
                 FROM ventas v
                 LEFT JOIN tipos_venta t ON t.id = v.id_tipo_venta
                 WHERE v.anulada = 0 AND v.fecha >= ?1 AND v.fecha < ?2
                 ORDER BY v.id_tipo_venta",
            )?;
            let mut rows = stmt.query(rusqlite::params![start, end])?;
            while let Some(row) = rows.next()? {
                let id_tipo: i64 = row.get(0)?;
                let nombre: String = row.get(1)?;
                let total: f64 = row.get(2)?;
                let entry = ventas_by_tipo.entry(id_tipo).or_insert((nombre, 0.0));
                entry.1 += total;
            }
        }

        let total_venta: f64 = ventas_by_tipo.values().map(|(_, total)| *total).sum();

        let mut total_costo = 0.0;
        {
            let mut stmt = conn.prepare(
                "SELECT d.costo_unitario * d.cantidad
                 FROM venta_detalle d
                 INNER JOIN ventas v ON v.id = d.id_venta
                 WHERE v.anulada = 0 AND v.fecha >= ?1 AND v.fecha < ?2",
            )?;
            let mut rows = stmt.query(rusqlite::params![start, end])?;
            while let Some(row) = rows.next()? {
                let costo: f64 = row.get(0)?;
                total_costo += costo;
            }
        }
        drop(conn);

        let total_costo = round2(total_costo);
        let total_venta = round2(total_venta);
        let total_ganancia = round2(total_venta - total_costo);

        let tipos: Vec<CierreTipo> = ventas_by_tipo
            .into_iter()
            .map(|(id_tipo, (tipo_venta, total))| CierreTipo {
                id_tipo_venta: id_tipo,
                tipo_venta,
                total: round2(total),
            })
            .collect();

        let cierre = Cierre {
            id: 0,
            fecha: fecha.to_string(),
            dia: day.day() as i64,
            mes: day.month() as i64,
            anio: day.year() as i64,
            total_costo,
            total_ganancia,
            total_venta,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        self.repository.create(&cierre, &tipos)
    }

    pub fn get_all(&self) -> Result<Vec<CierreWithTipos>, AppError> {
        self.repository.find_all()
    }
}

fn local_to_utc(dt: &chrono::NaiveDateTime) -> String {
    Local
        .from_local_datetime(dt)
        .single()
        .unwrap()
        .with_timezone(&chrono::Utc)
        .to_rfc3339()
}

fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
