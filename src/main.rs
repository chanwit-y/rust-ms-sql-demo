use rust_ms_sql_demo::utils::{ms_sql_tb, db::Database};
// use indicatif::{ProgressBar, ProgressStyle, ProgressState};
// use std::time::Duration;
// use tokio::runtime;
// use tokio::time::interval;
// use std::{ fmt::Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let db = Database::new().await;
    let mut ms_sql = ms_sql_tb::new(db).await;
    let res = ms_sql.get_table_names().await;
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

    // println!("---------------------------------");
    // let cols = get_all_colums("TRIP").await;
    // for col in cols {
    //     println!("{}", col.column_name);
    // }

    //  loading().await;

    Ok(())
}

//  struct A <'b>{
//     fname: String,
//     lname: String
// }

// fn test<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }

// fn testx<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }

//  async fn  loading() {
//     let steps = 1024;
//     let pb = ProgressBar::new(steps);
//     pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
//         .unwrap()
//         .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
//         .progress_chars("#>-"));

//     let rt = runtime::Builder::new_current_thread()
//         .enable_time()
//         .build()
//         .expect("failed to create runtime");

//     // Future computation which runs for `steps` interval events,
//     // incrementing one step of the progress bar each time.
//     let future = async {
//         let mut intv = interval(Duration::from_millis(5));

//         for _ in 0..steps {
//             intv.tick().await;
//             pb.inc(1);
//         }
//     };

//     // Drive the future to completion, blocking until done.
//     rt.block_on(future);

//     // Mark the progress bar as finished.
//     pb.finish();
// }
