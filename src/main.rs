extern crate actix_web;
extern crate askama;
extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate url;
use actix_web::actix::{SyncArbiter, System};
use actix_web::{http, server, App};
use clap::Arg;
use controllers::{hub, index};
use diesel::prelude::*;
use utils::{setup_logging, AppState, DbExecutor};

mod actions;
mod controllers;
mod models;
mod schema;
mod utils;

fn main() {
    let log = setup_logging();
    info!(log, "Launching hub");
    let matches = clap::App::new("Rusty Hub")
        .version("0.1.0")
        .author("Gonçalo Valério <gon@ovalerio.net>")
        .about("Runs a simple and compliant websub hub")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let address = "127.0.0.1";
    let port = "8888";

    info!(log, "Loading configuration");
    let config = matches.value_of("config").unwrap_or("");
    if !config.is_empty() {
        println!("Not implemented");
        return;
    }

    let sys = System::new("rusty-hub");
    let addr = SyncArbiter::start(3, || {
        DbExecutor(SqliteConnection::establish("local.db").unwrap())
    });

    info!(log, "Starting server");
    server::new(move || {
        App::with_state(AppState {
            log: setup_logging(),
            db: addr.clone(),
        })
        .route("/", http::Method::GET, index)
        .route("/", http::Method::POST, hub)
    })
    .bind(format!("{}:{}", address, port))
    .unwrap()
    .start();
    let _ = sys.run();
    info!(log, "Shutting down server");
}
