use crate::models::colums::TableColumn;

use super::{
    db::Database,
    sql::{query_colums, query_tables, query_foreign_key},
};

pub struct MsSqlTable {
    pub db: Database,
}

pub async fn new(_db: Database) -> MsSqlTable {
    MsSqlTable { db: _db }
}

impl MsSqlTable {
    pub async fn get_table_names(&mut self) -> Option<Vec<String>> {
        // let mut db = Database::new().await;
        let db = &mut self.db;
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

    pub async fn get_all_colums(&mut self, tb_name: &str) -> Vec<TableColumn> {
        // let mut db = Database::new().await;
        let db = &mut self.db;
        let rows = db.selelec_where(query_colums(), &[&tb_name]).await;
        let mut colums: Vec<TableColumn> = Vec::new();
        for row in rows {
            let column_name: &str = row.get(0).unwrap_or_default();
            let data_type: &str = row.get(1).unwrap_or_default();
            let character_maximum_length: i32 = row.get(2).unwrap_or_default();
            let column_default: &str = row.get(3).unwrap_or_default();
            let is_nullable: bool = row.get(4).unwrap_or_default();
            let is_identity: i32 = row.get(5).unwrap_or_default();
            let table_name: &str = row.get(6).unwrap_or_default();
            let constraint_name: &str = row.get(7).unwrap_or_default();
            let numeric_precision: u8 = row.get(8).unwrap_or_default();
            let numeric_scale: i32 = row.get(9).unwrap_or_default();

            let col: TableColumn = TableColumn {
                column_name: column_name.to_string(),
                data_type: data_type.to_string(),
                character_maximum_length,
                column_default: column_default.to_string(),
                is_nullable,
                is_identity,
                table_name: table_name.to_string(),
                constraint_name: constraint_name.to_string(),
                numeric_precision,
                numeric_scale,
            };
            colums.push(col);
        }

        colums
    }

    pub async fn get_foreign_keys(&mut self, tb_name: &str) {
        // let mut db = Database::new().await;
        let db = &mut self.db;
        let rows = db.selelec_where(query_foreign_key(), &[&tb_name]).await;

        for row in rows {
            
        }
    }
}

// pub async fn get_table_names() -> Option<Vec<String>> {
//     let mut db = Database::new().await;
//     let rows = db.sqlect(query_tables()).await;
//     let mut result: Vec<String> = Vec::new();
//     for row in rows {
//         let table_name: &str = row.get(0).unwrap_or_default();
//         result.push(table_name.to_string());
//     }

//     if result.len() == 0 {
//         None
//     } else {
//         Some(result)
//     }
// }

// pub async fn get_all_colums(tb_name: &str) -> Vec<TableColumn> {
//     let mut db = Database::new().await;
//     let rows = db.selelec_where(query_colums(), &[&tb_name]).await;
//     let mut colums: Vec<TableColumn> = Vec::new();
//     for row in rows {
//         let column_name: &str = row.get(0).unwrap_or_default();
//         let data_type: &str = row.get(1).unwrap_or_default();
//         let character_maximum_length: i32 = row.get(2).unwrap_or_default();
//         let column_default: &str = row.get(3).unwrap_or_default();
//         let is_nullable: bool = row.get(4).unwrap_or_default();
//         let is_identity: i32 = row.get(5).unwrap_or_default();
//         let table_name: &str = row.get(6).unwrap_or_default();
//         let constraint_name: &str = row.get(7).unwrap_or_default();
//         let numeric_precision: u8 = row.get(8).unwrap_or_default();
//         let numeric_scale: i32 = row.get(9).unwrap_or_default();

//         let col: TableColumn = TableColumn {
//             column_name: column_name.to_string(),
//             data_type: data_type.to_string(),
//             character_maximum_length: character_maximum_length,
//             column_default: column_default.to_string(),
//             is_nullable: is_nullable,
//             is_identity: is_identity,
//             table_name: table_name.to_string(),
//             constraint_name: constraint_name.to_string(),
//             numeric_precision: numeric_precision,
//             numeric_scale: numeric_scale,
//         };
//         colums.push(col);
//     }

//     colums
// }
