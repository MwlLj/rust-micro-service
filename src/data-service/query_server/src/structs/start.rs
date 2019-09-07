#[derive(Default, Clone, Debug)]
pub struct CCert {
}

#[derive(Default, Clone, Debug)]
pub struct CHttp<'a> {
    pub proto: String,
    pub ip: Option<&'a str>,
    pub port: u16,
    pub cert: Option<CCert>
}

#[derive(Default, Clone, Debug)]
pub struct CQueryStart<'a> {
    pub httpListen: Option<CHttp<'a>>,
    pub selectMode: String,
    pub sessionMode: String,
    pub guardMode: String,
    pub sessionDial: String,
    pub guardDial: String
}
