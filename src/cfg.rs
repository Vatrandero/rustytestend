use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub db_cfg: DbCfg,
    pub api_cfg: APICfg,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DbCfg {
    Postgresql(DBPGCfg),
}
impl DbCfg{
    // FIXME!!!!
    pub fn get_pg(&self ) -> Option<DBPGCfg> {
        match self{ 
            Self::Postgresql(r) => Some(r.clone()), 
            _ => None
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "pg")]
pub struct DBPGCfg {
    pub host: String,
    pub port: Option<u16>,
    pub db_name: String,
    pub use_tls: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "net")]
pub struct APICfg {
    pub api_addres_and_port: String,
    pub host_doc: bool, // TODO: Seperate openapi from all api.
}
impl Default for Config {
    fn default() -> Self {
        let db_pg = DbCfg::Postgresql(DBPGCfg {
            host: String::from("localhost"),
            db_name: String::from("Testing_system"),
            port: Some(5432),
            use_tls: false,
        });
        let ncfg = APICfg {
            api_addres_and_port: String::from("127.0.0.1:8080"),
            host_doc: true,
        };
        Self {
            db_cfg: db_pg,
            api_cfg: ncfg,
        }
    }
}
