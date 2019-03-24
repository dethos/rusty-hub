extern crate actix_web;
extern crate clap;
use clap::{Arg};
use actix_web::{server, App};

mod controllers;

fn main() {
    println!("[rustyhub] Launching hub");
    let matches = clap::App::new("Rusty Hub")
        .version("0.1.0")
        .author("Gonçalo Valério <gon@ovalerio.net>")
        .about("Runs a simple and compliant websub hub")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    let address = "127.0.0.1";
    let port = "8888";

    println!("[rustyhub] Loading configuration");
    let config = matches.value_of("config").unwrap_or("");
    if !config.is_empty() {
        println!("Not implemented");
        return
    }

    println!("[rustyhub] Starting server");
    server::new(|| App::new().resource("/", |r| r.f(controllers::index)))
        .bind(format!("{}:{}", address, port))
        .unwrap()
        .run();
    println!("[rustyhub] Shutting down server");
}
