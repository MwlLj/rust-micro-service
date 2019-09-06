#[derive(Default)]
pub struct CHttpHeart {
}

#[derive(Default)]
pub struct CHeart {
    pub http: Option<CHttpHeart>,
    pub intervalMs: Option<u64>
}
