use serde::{Deserialize, Serialize};

/* todo */
pub struct Undefined {

}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response <T>{
    code: i32,
    message: String,
    ttl: i32,
    pub data: T,
}
