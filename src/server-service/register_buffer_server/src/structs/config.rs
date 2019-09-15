use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CRegisterCenter {
    pub dial: String,
    pub proto: String
}
