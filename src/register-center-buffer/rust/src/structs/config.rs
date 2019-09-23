use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CRegisterCenter {
    pub dial: String,
    pub proto: String
}

