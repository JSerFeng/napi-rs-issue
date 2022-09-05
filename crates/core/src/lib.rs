use napi_derive::napi;
use plugin::Plugin;

#[derive(Debug)]
#[napi(object)]
pub struct Config {
    pub plugin: Plugin
}


#[napi]
pub fn run(config: Config) {
    println!("{:?}", config);
}
