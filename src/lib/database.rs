use tiberius::{Client, Config};

struct Database {
    client: Client,
}

impl Database {
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
}
