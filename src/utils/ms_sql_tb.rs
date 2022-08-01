use super::db::Database;

pub async fn get_table_names() -> Option<Vec<String>> {
    let mut db = Database::new().await;
    let sql = r#"
            SELECT tbl.name AS table_name
            FROM sys.tables tbl
            WHERE tbl.is_ms_shipped = 0
                AND tbl.type = 'U'
            ORDER BY tbl.name;
        "#;

    let rows = db.sqlect(sql).await;
    let mut result: Vec<String> = Vec::new();
    for row in rows {
        let table_name: &str = row.get(0).unwrap_or_default();
	result.push(table_name.to_string());
        // print!("{} \n", table_name);
    }

    if result.len() == 0 {
        None
    } else {
        Some(result)
    }
}
