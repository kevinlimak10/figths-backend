#![allow(unused)]

mod api;
mod cmd;
mod error;
mod prelude;

use crate::prelude::*;
use std::path::PathBuf;
use mysql::{Opts, OptsBuilder, Pool};

use structured_logger::{async_json::new_writer, Builder as LogBuilder};
use tokio::io;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};
use tonic::transport::Server as TonicServer;


#[tokio::main]
async fn main() -> Result<()> {
    LogBuilder::with_level("debug")
        .with_default_writer(new_writer(io::stdout()))
        .init();

    let port = 8080;

    let addr = format!("127.0.0.1:{}", port).parse()?;

    log::info!("running server on port {}", port);

    let (main_tx, app_rx) = oneshot::channel::<()>();
    let (app_tx, main_rx) = oneshot::channel::<()>();

    let app_task = tokio::spawn(run_app(app_tx, app_rx));

    TonicServer::builder()
        .add_service(api::spec_service()?)
        .add_service(api::extract_service())
        .add_service(api::transaction_service())
        .serve(addr)
        .await?;

    Ok(())
}

async fn run_app(tx: Sender<()>, rx: Receiver<()>) -> Result<()> {
   let pool = establish_connection();

    if tx.send(()).is_err() {
        log::warn!("main thread dropped receiever");
    }
    log::info!("application shutdown gracefully");
    Ok(())
}

fn establish_connection() -> Pool {
    let url = "mysql://root:password@localhost:3306/db_name";
    // MySQL connection options
    Pool::new(url)?
}