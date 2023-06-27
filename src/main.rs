//#![allow(dead_code, unused)]
mod args;
mod auth;
mod config;
mod responses;
mod server;

use std::sync::OnceLock;

use args::Cli;
use auth::AuthManager;
use clap::Parser;
use config::Configuration;
use rocket::{launch, routes};
use server::*;

static CONFIG: OnceLock<Configuration> = OnceLock::new();
static CLI: OnceLock<Cli> = OnceLock::new();
static AUTH: OnceLock<AuthManager> = OnceLock::new();

#[launch]
async fn rocket() -> _ {
    let cli = Cli::parse();
    let config = Configuration::load(&cli.config).unwrap();
    AUTH.set(AuthManager::from(&config)).unwrap();
    CONFIG.set(config).unwrap();
    CLI.set(cli).unwrap();

    rocket::build().mount("/", routes![index, list, download, upload, download_part])
}
