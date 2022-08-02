use crate::models::colums::TableColumn;

use super::{
    db::Database,
    sql::{query_colums, query_tables},
};

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

pub async fn get_all_colums<'a>(tb_name: &str) {
    let mut db = Database::new().await;
    let rows = db.selelec_where(query_colums(), &[&tb_name]).await;
    let mut colums: Vec<TableColumn> = Vec::new();
    for row in rows {
        let column_name: &str = row.get(0).unwrap_or_default();
        let data_type: &str = row.get(1).unwrap_or_default();
        let character_maximum_length: i32 = row.get(2).unwrap_or_default();
        let column_default: &str = row.get(3).unwrap_or_default();
        let is_nullable: bool = row.get(4).unwrap_or_default();
        let is_identity: bool = row.get(5).unwrap_or_default();
        let table_name: &str = row.get(6).unwrap_or_default();

        let col: TableColumn = TableColumn {
            column_name: column_name.to_string(),
            data_type: data_type.to_string(),
            character_maximum_length: character_maximum_length,
            column_default: column_default.to_string(),
            is_nullable: is_nullable,
            is_identity: is_identity,
            table_name: table_name.to_string(),
        };

        println!("{}, {}, {}", col.column_name, col.data_type, col.table_name);

        colums.push(col);
    }
}
