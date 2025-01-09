use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub db_cfg: db_cfg,
    pub net_cfg: net_cfg,
}

#[derive(Debug,Deserialize,Serialize)]
pub enum db_cfg{
    Postgresql(db_pg_cfg),    
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename="pg")]
pub struct db_pg_cfg {
    pub host: String, 
    pub port: Option<u16>, 
    pub db_name: String,
    pub use_tls: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "net")]
pub struct net_cfg {
    pub addreses: Vec<String>,
    pub port: usize,
}
impl Default for Config {
    fn default() -> Self {
        let db_pg = db_cfg::Postgresql( db_pg_cfg {
            host: String::from("localhost"),
            db_name: String::from("Testing_system"),
            port: Some(5432),
            use_tls: false
        });
        let ncfg = net_cfg {
            addreses: vec![String::from("127.0.0.1")],
            port: 58910,
        };
        Self {
            db_cfg: db_pg,
            net_cfg: ncfg,
        }
    }
}
