mod api;
pub mod cfg;
pub mod db;
pub mod models;
use axum::routing::trace;
use cfg::DbCfg;
use clap::Parser;
use std::{
    io::{Read, Write},
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};

use tokio::runtime::{Builder, Runtime};

extern crate env_logger;
#[macro_use]
extern crate log;

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub gen_config: Option<PathBuf>, // Actualy, it only prints it now.

    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    // Should we generate default config?
    if let Some(p) = args.gen_config {
        let default_config = toml::to_string(&cfg::Config::default()).unwrap();
        print!("{}", default_config);
        return;
    }
    // Early debug logging.
    let mut lbuilder = env_logger::builder();

    lbuilder.filter_level(log::LevelFilter::Trace);

    lbuilder.init();

    // TODO: add logging to config
    // TODO: OR: init with env.

    trace!("Logger initialized, trace.");

    // Can we load config?
    let cfg_path = match args.config {
        Some(cfgp) => {
            if cfgp.is_file() {
                cfgp
            } else {
                eprint!("Bad config path? trying for default rustytestend.toml");
                PathBuf::from("rustytestend.toml")
            }
        }
        None => PathBuf::from("rustytestend.toml"),
    };
    let cfg: cfg::Config = match std::fs::read_to_string(cfg_path) {
        Ok(cfgf) => match toml::from_str(cfgf.as_str()) {
            Ok(r) => r,
            Err(e) => panic!("Bad config file? \n {}", e),
        },
        Err(e) => {
            print!("Failed to read config, can't go. {}", e);
            panic!("Failed to read config file.")
        }
    };

    // let's build our router

    // Okay, we ready to start runtime and connections.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(3)
        .build()
        .unwrap();

    // Builded, ready to go
    rt.block_on(async {
        let db = Arc::new((db::DBPostgres::try_init(&cfg.db_cfg.get_pg().unwrap()).await).unwrap());
        let state = api::AppState {
            dbpool_user_manager: db.clone(),
            // As long as we use the same driver for anything....
            // We just clone our Arc.
            dbpool_session_manager: db.clone(),
        };

        trace!("Reached: async runtime runed");

        let api_router = api::init_router(&cfg, state);
        let mut socketsaddrs: Vec<SocketAddr> = Vec::new();
        let ncfg = &cfg.api_cfg.clone();
        let addr: std::net::SocketAddr = match ncfg.api_addres_and_port.parse() {
            Ok(r) => r,
            Err(_) => {
                panic!("failed to parse host addrss")
            }
        };

        axum::serve(
            tokio::net::TcpListener::bind(addr).await.unwrap(),
            api_router,
        )
        .await
        .unwrap();
    })
}
