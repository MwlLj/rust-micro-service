use serde::{Serialize, Deserialize};

use service_select_client::structs;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CClientParam {
    pub selectClientParam: structs::start::CClientParam,
    pub registerBufferServerName: String
}
