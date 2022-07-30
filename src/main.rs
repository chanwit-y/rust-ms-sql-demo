// extern crate futures;
// extern crate tiberius;
// extern crate tokio_core;

// use std::result::{Result};
// use tiberius::AuthMethod;

mod database;
use tiberius::{Client};

// use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
// use tiberius::stmt::ResultStreamExt;

// static CONN_STR: Lazy<String> = Lazy::new(|| {
//     env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
//         "server=tcp:localhost,1433;IntegratedSecurity=true;TrustServerCertificate=true".to_owned()
//     })
// });

// #[cfg(not(all(windows, feature = "sql-browser-tokio")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let config = get_config();
    // let tcp = get_connection(&config).await.unwrap();
    // let db: Database::new();
    let config = database::get_config();
    let tcp = database::get_connection(&config).await.unwrap();

    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;
    let sql = r#"
            SELECT tbl.name AS table_name
            FROM sys.tables tbl
            WHERE tbl.is_ms_shipped = 0
                AND tbl.type = 'U'
            ORDER BY tbl.name;
        "#;

    let stream = client.simple_query(sql).await?;
    let rows = stream.into_first_result().await?;

    for row in rows {
        let table_name: &str = row.get(0).unwrap_or_default();
        print!("{} \n", table_name);
    }

    Ok(())
}

// fn get_config() -> Config {
//     let mut config = Config::new();

//     config.host("0.0.0.0");
//     config.port(1433);
//     config.database("demo");
//     config.authentication(AuthMethod::sql_server("SA", "abcABC123"));

//     config
// }

// async fn get_connection(config: &Config) -> Result<TcpStream, std::io::Error>  {
//     let tcp = TcpStream::connect(config.get_addr()).await;
//     tcp
// }
