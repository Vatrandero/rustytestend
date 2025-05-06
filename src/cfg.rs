use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general_cfg: GCfg,
    pub db_cfg: DbCfg,
    pub api_cfg: APICfg,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GCfg{
    pub runtime_thrads: i32, 
    pub bactrace_panic_logs: bool    
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "pg")]
pub struct DBPGCfg {
    pub host: String,
    pub port: Option<u16>,
    pub db_name: String,
    pub username: String, 
    pub password: String,
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
        let gcfg = GCfg { 
            runtime_thrads: 3, 
            bactrace_panic_logs: false,
        };      
        let db_pg = DbCfg::Postgresql(DBPGCfg {
            host: String::from("localhost"),
            db_name: String::from("Testing_system"),
            username: String::from("postgres"),
            password: String::from(""),
            port: Some(5432),
            use_tls: false,
        });
        let ncfg = APICfg {
            api_addres_and_port: String::from("127.0.0.1:8080"),
            host_doc: true,
        };
        Self {
            general_cfg: gcfg,
            db_cfg: db_pg,
            api_cfg: ncfg,
        }
    }
}
