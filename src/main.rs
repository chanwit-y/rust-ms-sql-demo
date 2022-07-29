use tiberius::AuthMethod;
// use once_cell::sync::Lazy;
// use std::env;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

// static CONN_STR: Lazy<String> = Lazy::new(|| {
//     env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
//         "server=tcp:localhost,1433;IntegratedSecurity=true;TrustServerCertificate=true".to_owned()
//     })
// });

// #[cfg(not(all(windows, feature = "sql-browser-tokio")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let config = Config::from_ado_string(&CONN_STR)?;

    let mut config = Config::new();

    config.host("0.0.0.0");
    config.port(1433);
    config.database("demo");
    config.authentication(AuthMethod::sql_server("SA", "abcABC123"));
    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let stream = client.query("SELECT @P1 as c1", &[&1i32]).await?;
    let row = stream.into_row().await?.unwrap();

    println!("{:?}", row);
    // assert_eq!(Some(1), row.get(0));

    Ok(())
}