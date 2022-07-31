use rust_ms_sql_demo::utils::db::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut db = Database::new().await;
    let sql = r#"
            SELECT tbl.name AS table_name
            FROM sys.tables tbl
            WHERE tbl.is_ms_shipped = 0
                AND tbl.type = 'U'
            ORDER BY tbl.name;
        "#;
        
    let rows = db.sqlect(sql).await;
    for row in rows {
        let table_name: &str = row.get(0).unwrap_or_default();
        print!("{} \n", table_name);
    }

    Ok(())
}
