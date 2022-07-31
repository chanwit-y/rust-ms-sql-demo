
use rust_ms_sql_demo::utils::db::{ Database};
// use tiberius::Client;

// use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let config = get_config();
    // let tcp = get_connection(&config).await.unwrap();
    // tcp.set_nodelay(true)?;

    // let mut client = Client::connect(config, tcp.compat_write()).await?;
    let mut db = Database::new().await;
    let sql = r#"
            SELECT tbl.name AS table_name
            FROM sys.tables tbl
            WHERE tbl.is_ms_shipped = 0
                AND tbl.type = 'U'
            ORDER BY tbl.name;
        "#;
    db.sqlect(sql).await;
    

    // let stream = client.simple_query(sql).await?;
    // let rows = stream.into_first_result().await?;

    // for row in rows {
    //     let table_name: &str = row.get(0).unwrap_or_default();
    //     print!("{} \n", table_name);
    // }

    Ok(())
}
