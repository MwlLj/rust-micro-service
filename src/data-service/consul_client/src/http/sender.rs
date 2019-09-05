use reqwest::{Client, Response, Error};
use serde::Serialize;

pub struct CSender {
    client: Client,
    addr: String
}

impl CSender {
    pub fn putJsonObject<T>(&self, url: &str, t: &T) -> Result<Response, Error>
        where T: Serialize {
        let mut uri = String::new();
        uri.push_str(&self.addr);
        uri.push_str(url);
        self.client.put(&uri).json(t).send()
    }

    pub fn get<Q>(&self, url: &str, query: Option<&Q>) -> Result<Response, Error>
        where Q: Serialize {
        let mut uri = String::new();
        uri.push_str(&self.addr);
        uri.push_str(url);
        let mut cli = self.client.get(&uri);
        match query {
            Some(q) => {
                cli = cli.query(q);
            },
            None => {
            }
        }
        cli.send()
    }
}

impl CSender {
    pub fn new(addr: &str) -> Option<CSender> {
        Some(CSender{
            client: Client::new(),
            addr: addr.to_owned()
        })
    }
}
