use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StockBajoItem {
    pub id_stock: i64,
    pub id_articulo: i64,
    pub cod_articulo: String,
    pub articulo: String,
    pub cantidad: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubCategoriaInfo {
    pub id: i64,
    pub sub_categoria: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoriaConSub {
    pub id: i64,
    pub categoria: String,
    pub sub_categorias: Vec<SubCategoriaInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HomeStats {
    pub total_articulos: i64,
    pub articulos_con_stock: i64,
    pub total_usuarios: i64,
    pub usuarios_activos: i64,
    pub usuarios_inactivos: i64,
    pub total_proveedores: i64,
    pub total_categorias: i64,
    pub total_sub_categorias: i64,
    pub ventas_hoy: i64,
    pub total_ventas_hoy: f64,
    pub stock_bajo: Vec<StockBajoItem>,
    pub categorias: Vec<CategoriaConSub>,
}
