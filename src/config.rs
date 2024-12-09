use serde::{Serialize, Deserialize}; 

#[derive(Debug, Serialize, Deserialize )]
pub struct Config {
    db: db_cfg, 
    net: net_cfg
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename="db")]
struct db_cfg{
postgres_url: String, 
tls_enabled: bool
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "net")]
struct net_cfg {
    ip_addreses: Vec<String>, 
    port: usize

}