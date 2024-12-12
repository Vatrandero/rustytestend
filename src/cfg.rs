use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    db_cfg: db_pg_cfg,
    net_cfg: net_cfg,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "db.posrgresql")]
struct db_pg_cfg {
    postgres_url: String,
    use_tls: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "net")]
struct net_cfg {
    addreses: Vec<String>,
    port: usize,
}
impl Default for Config {
    fn default() -> Self {
        let db_pg = db_pg_cfg {
            postgres_url: "postgresql://user:password@localhost/dbname".to_string(),
            use_tls: false,
        };
        let ncfg = net_cfg {
            addreses: vec![String::from("127.0.0.1")],
            port: 8001,
        };
        Self {
            db_cfg: db_pg,
            net_cfg: ncfg,
        }
    }
}
