use rust_ms_sql_demo::utils::ms_sql_tb::get_table_names;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = get_table_names().await;
    let mut tb_names: Vec<String> = Vec::new();
    match res {
        Some(r) => {
            tb_names = r.to_owned();
        }
        None => println!("data not found"),
    }

    for tb in &tb_names {
        println!("{}", tb)
    }
    // if tb_names != null {
    // tb_names.iter().map(|x| println!("{}", x));
    // }

    Ok(())
}
