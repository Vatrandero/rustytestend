mod knowledge_test;
mod api; 
mod cfg; 
fn main() { 
   let default_config = toml::to_string( &cfg::Config::default()).unwrap();
    print!("{}",default_config);
}