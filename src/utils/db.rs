use std::result::Result;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

fn get_config() -> Config {
    let mut config = Config::new();

    config.host("0.0.0.0");
    config.port(1433);
    config.database("demo");
    config.authentication(AuthMethod::sql_server("SA", "abcABC123"));

    config
}

async fn get_connection(config: &Config) -> Result<TcpStream, std::io::Error> {
    let tcp = TcpStream::connect(config.get_addr()).await;
    tcp
}

pub struct Database {
    client: Client<Compat<TcpStream>>,
}

impl Database {
    pub async fn new() -> Self {
        let config = get_config();
        let tcp = get_connection(&config).await.unwrap();
        tcp.set_nodelay(true).unwrap();

        Self {
            client: Client::connect(config, tcp.compat_write()).await.unwrap(),
        }
    }

    pub async fn sqlect(&mut self, sql: &str) {
        let client = &mut self.client;
        let stream = client.simple_query(sql).await.unwrap();
        let rows = stream.into_first_result().await.unwrap();

        for row in rows {
            let table_name: &str = row.get(0).unwrap_or_default();
            print!("{} \n", table_name);
        }
    }
}

// impl<'a> Default for Database<'a> {
//     fn default() -> Self {
//         Self::new()
//     }
// }
