//#![allow(dead_code, unused)]
mod args;
mod config;
mod responses;
mod server;

use std::sync::OnceLock;

use args::Cli;
use clap::Parser;
use config::Configuration;
use rocket::{launch, routes};
use server::*;

static CONFIG: OnceLock<Configuration> = OnceLock::new();
static CLI: OnceLock<Cli> = OnceLock::new();

#[launch]
async fn rocket() -> _ {
    let cli = Cli::parse();
    let config = Configuration::load(&cli.config).unwrap();
    CONFIG.set(config).unwrap();
    CLI.set(cli).unwrap();

    rocket::build().mount("/", routes![index, list, download, upload, download_part])
}
