mod api;
pub mod cfg;
pub mod db;
pub mod models;
use axum::routing::trace;
use cfg::DbCfg;
use clap::Parser;
use std::{
    fs::File, io::{Read, Write}, net::{IpAddr, SocketAddr}, path::PathBuf, sync::Arc
};
use tokio::runtime::{Builder, Runtime};

extern crate env_logger;
#[macro_use]
extern crate log;

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub gen_config: Option<PathBuf>, //FIXME: Actualy, only prints it now.

    #[arg(short, long, value_name = "FILE")]
    
    pub config: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    // Should we generate default config?
    if let Some(p) = args.gen_config {
        println!("Default config will be generated into {}", p.clone().to_str().unwrap());
        let default_config = toml::to_string(&cfg::Config::default()).unwrap();
        let mut f = match  File::create_new(p) { 
            Ok(ncfgf) =>  ncfgf,
            Err(e) => {panic!("Bad path or file for new conffig. {}", e)}
        };
        f.write_all(default_config.as_bytes()).unwrap();
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

    // let's install log-panics hook.
    let is_backtrace = std::env::var("RUST_BACKTRACE")
    .unwrap_or_else(|_|   {"false".to_string()});
    log_panics::Config::new()
    .backtrace_mode(
        if is_backtrace.eq_ignore_ascii_case("true") || is_backtrace == "1" 
        {log_panics::BacktraceMode::Resolved}
        else{log_panics::BacktraceMode::Off}
    ).install_panic_hook();

    // Okay, we ready to start runtime and connections.
    
    // If we debuggine - single-thread is eaasier to debug.
    #[cfg(debug_assertions)]
    let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();
   #[cfg(debug_assertions)]
    debug!("Tokio-runtime launched single-thread for debug purpose");

    #[cfg(not(debug_assertions))]
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(cfg.general_cfg.runtime_thrads.try_into().
        unwrap_or_else(|_|
             {error!("Can't use threads from cfg. using 3 as default."); 3 as usize}))
        .build()
        .unwrap();
    // Builded, ready to go
    rt.block_on(async {
        let db = Arc::new(
            (db::pgsql::DBPostgres::try_init(&cfg.db_cfg.get_pg().unwrap()).await)
            .unwrap());
        let state = api::AppState {
            user_manager: db.clone(),
            // As long as all managers use the same underlying driver,
            // we can safely clone the Arc for shared access.
            user_session_manager: db.clone(),
            ktest_manager: db.clone(),
            ktest_session_manager: db.clone()
            // TODO: Replace with flixible and extenisable builder.
        };
        trace!("Reached: async runtime runed");

        let api_router = api::init_router(&cfg, state);
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
