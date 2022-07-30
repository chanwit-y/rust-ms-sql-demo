use std::result::Result;
use tiberius::{AuthMethod, Config};
use tokio::net::TcpStream;
// use tokio_util::compat::TokioAsyncWriteCompatExt;

// pub struct Database {
//     //  client: Client,
// }

// impl Database {
    pub fn get_config() -> Config {
        let mut config = Config::new();

        config.host("0.0.0.0");
        config.port(1433);
        config.database("demo");
        config.authentication(AuthMethod::sql_server("SA", "abcABC123"));

        config
    }

    pub async fn get_connection(config: &Config) -> Result<TcpStream, std::io::Error> {
        let tcp = TcpStream::connect(config.get_addr()).await;
        tcp
    }
// }
