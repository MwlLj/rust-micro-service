use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CClientParam {
    pub selectMode: String,
    pub sessionMode: String,
    pub sessionDial: String,
}
