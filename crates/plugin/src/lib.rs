use napi_derive::napi;

#[napi(object)]
#[derive(Debug)]
pub struct Plugin {
    pub msg: String    
}
