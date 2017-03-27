use std::io::Read;
use hyper::Client;
use hyperlocal::{DomainUrl, UnixSocketConnector};

pub struct Binding {
    endpoint: &'static str,
}

impl Binding {
    pub fn new(endpoint: &'static str) -> Binding {
        Binding { endpoint: endpoint }
    }

    pub fn get(&self, api: &str) -> String {
        let url = self.endpoint;
        get_docker_api(url, api)
    }

    pub fn post(&self, api: &str, data: &str) -> String {
        let url = self.endpoint;
        post_docker_api(url, api, data)
    }
}

fn get_docker_api(url: &str, api: &str) -> String {
    let client = Client::with_connector(UnixSocketConnector);

    let mut response = match client.get(DomainUrl::new(url, api)).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    return buf
}

fn post_docker_api(url: &str, api: &str, data: &str) -> String {
    let client = Client::with_connector(UnixSocketConnector);

    let mut response = match client.post(DomainUrl::new(url, api)).body(data).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    return buf
}
