use tiberius::{AuthMethod, Client, Config, Query};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print!("Hi");
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("sa", "abcABC123"));
    config.trust_cert(); // on production, it is not a good idea to do this

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;


    print!("start");
    // To be able to use Tokio's tcp, we're using the `compat_write` from
    // the `TokioAsyncWriteCompatExt` to get a stream compatible with the
    // traits from the `futures` crate.
    let mut client = Client::connect(config, tcp.compat_write()).await?;

    print!("end");

    // let mut select = Query::new("SELECT @P1");
    // select.bind(-4i32);
    // let stream = select.query(&mut client).await?;
    // let row = stream.into_row().await?.unwrap();

    // print!("{}", row.columns()[0].name());
    // for r in row {

    // }

    Ok(())
}
