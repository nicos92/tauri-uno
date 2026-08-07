use rusqlite::params;

use crate::domain::entities::{
    Venta, VentaDetalle, VentaDetalleConArticulo, VentaWithDetalle,
};
use crate::domain::repositories::VentaRepository;
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteVentaRepository;

impl SqliteVentaRepository {
    pub fn new() -> Self {
        Self
    }
}

impl VentaRepository for SqliteVentaRepository {
    fn create(
        &self,
        venta: &Venta,
        detalles: &[VentaDetalle],
        allow_negative_stock: bool,
    ) -> Result<VentaWithDetalle, AppError> {
        let mut conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        let tx = conn.transaction()?;

        let mut items: Vec<VentaDetalle> = Vec::new();
        let mut total = 0.0;

        for detalle in detalles {
            let stock = tx.query_row(
                "SELECT cantidad, costo, ganancia FROM stock WHERE id_articulo = ?1",
                params![detalle.id_articulo],
                |row| {
                    Ok((
                        row.get::<_, f64>(0)?,
                        row.get::<_, f64>(1)?,
                        row.get::<_, f64>(2)?,
                    ))
                },
            );

            let (stock_cantidad, costo, ganancia) = match stock {
                Ok(values) => values,
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    return Err(AppError::ArticuloWithoutStock);
                }
                Err(e) => return Err(e.into()),
            };

            if !allow_negative_stock && detalle.cantidad > stock_cantidad {
                return Err(AppError::InsufficientStock);
            }

            let precio_unitario = if detalle.precio_unitario > 0.0 {
                detalle.precio_unitario
            } else {
                costo * (1.0 + ganancia / 100.0)
            };

            let mut item = detalle.clone();
            item.costo_unitario = costo;
            item.precio_unitario = precio_unitario;
            item.subtotal = item.cantidad * item.precio_unitario;
            total += item.subtotal;
            items.push(item);
        }

        let now = chrono::Utc::now().to_rfc3339();
        total = (total * (1.0 - venta.descuento / 100.0) * 100.0).round() / 100.0;
        tx.execute(
            "INSERT INTO ventas (user_id, fecha, total, descuento, anulada, observacion, created_at) VALUES (?1, ?2, ?3, ?4, 0, ?5, ?6)",
            params![venta.user_id, now, total, venta.descuento, &venta.observacion, now],
        )?;
        let venta_id = tx.last_insert_rowid();

        for item in &items {
            tx.execute(
                "INSERT INTO venta_detalle (id_venta, id_articulo, cantidad, costo_unitario, precio_unitario, subtotal) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    venta_id,
                    item.id_articulo,
                    item.cantidad,
                    item.costo_unitario,
                    item.precio_unitario,
                    item.subtotal
                ],
            )?;

            tx.execute(
                "UPDATE stock SET cantidad = cantidad - ?1 WHERE id_articulo = ?2",
                params![item.cantidad, item.id_articulo],
            )?;
        }

        tx.commit()?;

        let mut venta = self
            .load_venta(&conn, venta_id)?
            .ok_or(AppError::VentaNotFound)?;
        venta.items = self.load_items(&conn, venta_id)?;
        venta.subtotal = venta.items.iter().map(|i| i.subtotal).sum();
        Ok(venta)
    }

    fn find_by_id(&self, id: i64) -> Result<Option<VentaWithDetalle>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut venta = self.load_venta(&conn, id)?;
        if let Some(v) = venta.as_mut() {
            v.items = self.load_items(&conn, id)?;
            v.subtotal = v.items.iter().map(|i| i.subtotal).sum();
        }
        Ok(venta)
    }

    fn find_all(&self) -> Result<Vec<VentaWithDetalle>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT v.id, v.user_id, COALESCE(u.username, ''), v.fecha, v.total, v.descuento, v.anulada, v.observacion, v.created_at
             FROM ventas v
             LEFT JOIN users u ON u.id = v.user_id
             ORDER BY v.id DESC",
        )?;

        let mut rows = stmt.query([])?;
        let mut ventas = Vec::new();

        while let Some(row) = rows.next()? {
            let mut venta = self.row_to_venta(row)?;
            venta.items = self.load_items(&conn, venta.id)?;
            venta.subtotal = venta.items.iter().map(|i| i.subtotal).sum();
            ventas.push(venta);
        }

        Ok(ventas)
    }

    fn anular(&self, id: i64) -> Result<(), AppError> {
        let mut conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        let tx = conn.transaction()?;

        let anulada: bool = tx
            .query_row("SELECT anulada FROM ventas WHERE id = ?1", params![id], |row| {
                row.get(0)
            })
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => AppError::VentaNotFound,
                other => other.into(),
            })?;

        if anulada {
            return Err(AppError::VentaAlreadyAnulada);
        }

        {
            let mut stmt =
                tx.prepare("SELECT id_articulo, cantidad FROM venta_detalle WHERE id_venta = ?1")?;
            let mut rows = stmt.query(params![id])?;

            while let Some(row) = rows.next()? {
                let id_articulo: i64 = row.get(0)?;
                let cantidad: f64 = row.get(1)?;

                let updated = tx.execute(
                    "UPDATE stock SET cantidad = cantidad + ?1 WHERE id_articulo = ?2",
                    params![cantidad, id_articulo],
                )?;
                if updated == 0 {
                    return Err(AppError::ArticuloWithoutStock);
                }
            }
        }

        tx.execute("UPDATE ventas SET anulada = 1 WHERE id = ?1", params![id])?;
        tx.commit()?;
        Ok(())
    }
}

impl SqliteVentaRepository {
    fn row_to_venta(&self, row: &rusqlite::Row) -> Result<VentaWithDetalle, AppError> {
        Ok(VentaWithDetalle {
            id: row.get(0)?,
            user_id: row.get(1)?,
            username: row.get(2)?,
            fecha: row.get(3)?,
            subtotal: 0.0,
            descuento: row.get(5)?,
            total: row.get(4)?,
            anulada: row.get(6)?,
            observacion: row.get(7)?,
            created_at: row.get(8)?,
            items: Vec::new(),
        })
    }

    fn load_venta(
        &self,
        conn: &rusqlite::Connection,
        id: i64,
    ) -> Result<Option<VentaWithDetalle>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT v.id, v.user_id, COALESCE(u.username, ''), v.fecha, v.total, v.descuento, v.anulada, v.observacion, v.created_at
             FROM ventas v
             LEFT JOIN users u ON u.id = v.user_id
             WHERE v.id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_venta(row)?))
        } else {
            Ok(None)
        }
    }

    fn load_items(
        &self,
        conn: &rusqlite::Connection,
        venta_id: i64,
    ) -> Result<Vec<VentaDetalleConArticulo>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT d.id, d.id_articulo, COALESCE(a.cod_articulo, ''), COALESCE(a.articulo, ''), d.cantidad, d.costo_unitario, d.precio_unitario, d.subtotal
             FROM venta_detalle d
             LEFT JOIN articulos a ON a.id = d.id_articulo
             WHERE d.id_venta = ?1
             ORDER BY d.id",
        )?;

        let mut rows = stmt.query(params![venta_id])?;
        let mut items = Vec::new();

        while let Some(row) = rows.next()? {
            items.push(VentaDetalleConArticulo {
                id: row.get(0)?,
                id_articulo: row.get(1)?,
                cod_articulo: row.get(2)?,
                articulo: row.get(3)?,
                cantidad: row.get(4)?,
                costo_unitario: row.get(5)?,
                precio_unitario: row.get(6)?,
                subtotal: row.get(7)?,
            });
        }

        Ok(items)
    }
}
