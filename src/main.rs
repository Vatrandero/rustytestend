mod api;
mod cfg;
mod knowledge_test;
fn main() {
    let default_config = toml::to_string(&cfg::Config::default()).unwrap();
    print!("{}", default_config);
}
