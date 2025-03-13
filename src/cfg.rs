use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub db_cfg: DbCfg,
    pub api_cfg: api_cfg,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DbCfg {
    Postgresql(db_pg_cfg),
}
impl DbCfg{
    // FIXME!!!!
    pub fn get_pg(&self ) -> Option<db_pg_cfg> {
        match self{ 
            Self::Postgresql(r) => Some(r.clone()), 
            _ => None
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "pg")]
pub struct db_pg_cfg {
    pub host: String,
    pub port: Option<u16>,
    pub db_name: String,
    pub use_tls: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "net")]
pub struct api_cfg {
    pub api_addres_and_port: String,
    pub host_doc: bool, // TODO: Seperate openapi from all api.
}
impl Default for Config {
    fn default() -> Self {
        let db_pg = DbCfg::Postgresql(db_pg_cfg {
            host: String::from("localhost"),
            db_name: String::from("Testing_system"),
            port: Some(5432),
            use_tls: false,
        });
        let ncfg = api_cfg {
            api_addres_and_port: String::from("127.0.0.1:8080"),
            host_doc: true,
        };
        Self {
            db_cfg: db_pg,
            api_cfg: ncfg,
        }
    }
}
