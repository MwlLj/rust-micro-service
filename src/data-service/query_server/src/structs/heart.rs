use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct CHttpHeart {
}

#[derive(Default, Serialize, Deserialize)]
pub struct CHeart {
    pub http: Option<CHttpHeart>,
    pub intervalMs: Option<u64>
}
