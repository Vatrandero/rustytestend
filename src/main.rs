mod api;
pub mod cfg;
pub mod knowledge_test;
pub mod users;
mod db;
fn main() {
    let default_config = toml::to_string(&cfg::Config::default()).unwrap();
    print!("{}", default_config);
}
