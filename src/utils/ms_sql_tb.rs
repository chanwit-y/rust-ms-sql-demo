use super::{db::Database, sql::{query_tables, query_colums}};

pub async fn get_table_names() -> Option<Vec<String>> {
    let mut db = Database::new().await;
    let rows = db.sqlect(query_tables()).await;
    let mut result: Vec<String> = Vec::new();
    for row in rows {
        let table_name: &str = row.get(0).unwrap_or_default();
        result.push(table_name.to_string());
    }

    if result.len() == 0 {
        None
    } else {
        Some(result)
    }
}

pub async fn get_all_colums(tb_name: &str) {
    let mut db = Database::new().await;
    let rows = db.selelec_where(query_colums(), &[&tb_name]).await;

    for row in rows {
        let column_name: &str = row.get(0).unwrap_or_default();
        let data_type: &str = row.get(1).unwrap_or_default();
        println!("{} {}", column_name, data_type);
    }
}
