#![allow(unused)]

mod api;
mod cmd;
mod error;
mod prelude;

use crate::prelude::*;
use std::path::PathBuf;

use structured_logger::{async_json::new_writer, Builder as LogBuilder};
use tokio::io;
use tonic::transport::Server as TonicServer;


#[tokio::main]
async fn main() -> Result<()> {
    // let args = cmd::parse();
    // let conf = config::load(PathBuf::from(args.config_path))?;

    LogBuilder::with_level("debug")
        .with_default_writer(new_writer(io::stdout()))
        .init();

    let port = 8080;

    let addr = format!("127.0.0.1:{}", port).parse()?;

    log::info!("running server on port {}", port);

    let (main_tx, app_rx) = oneshot::channel::<()>();
    let (app_tx, main_rx) = oneshot::channel::<()>();

    let app_task = tokio::spawn(run_app(conf, app_tx, app_rx));

    TonicServer::builder()
        .add_service(api::spec_service()?)
        .add_service(api::extract_service())
        .add_service(api::transaction_service())
        .serve(addr)
        .await?;

    Ok(())
}

async fn run_app(conf: config::AppConfig, tx: Sender<()>, rx: Receiver<()>) -> Result<()> {
   let pool = establish_connection();

    if tx.send(()).is_err() {
        log::warn!("main thread dropped receiever");
    }
    log::info!("application shutdown gracefully");
    Ok(())
}

fn establish_connection() -> Pool {
    // MySQL connection options
    let opts = Opts::from(
        OptsBuilder::new()
            .ip_or_hostname("localhost") // Your MySQL server IP or hostname
            .user("username")            // Your MySQL username
            .pass("password")            // Your MySQL password
            .db_name("database_name"),   // Your MySQL database name
    );

    // Create a MySQL connection pool
    Pool::new(opts).expect("Failed to create MySQL connection pool")
}