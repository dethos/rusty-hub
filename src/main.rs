extern crate actix;
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
use actix::System;
use actix_web::{web, App, HttpServer};
use clap::Arg;
use controllers::{hub, index};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use utils::{setup_logging, AppState};

mod actions;
mod controllers;
mod models;
mod schema;
mod utils;

fn main() {
    let matches = clap::App::new("Rusty Hub")
        .version("0.1.0")
        .author("Gonçalo Valério <gon@ovalerio.net>")
        .about("Runs a simple and compliant websub hub")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Set a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let log = setup_logging();
    info!(log, "Launching hub");

    let address = "127.0.0.1";
    let port = "8888";
    let storage = "local.db";

    info!(log, "Loading configuration");
    let config = matches.value_of("config").unwrap_or("");
    if !config.is_empty() {
        error!(log, "Configuration not implemented yet");
        return;
    }

    let sys = System::new("rusty-hub");
    let manager = ConnectionManager::<SqliteConnection>::new(storage);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let app_data = web::Data::new(AppState {
        log: setup_logging(),
        db: pool.clone(),
    });

    info!(log, "Starting server");
    HttpServer::new(move || {
        App::new()
            .register_data(app_data.clone())
            .route("/", web::get().to(index))
            .route("/", web::post().to(hub))
    })
    .bind(format!("{}:{}", address, port))
    .unwrap()
    .start();
    let _ = sys.run();
    info!(log, "Shutting down server");
}
