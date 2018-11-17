use std::net::TcpStream;
use super::methods::Method;
use super::request::Request;

pub struct Client {
    base_url: String
}

impl Client {
    pub fn new(base_url: String) -> Client {
        Client {
            base_url
        }
    }

    fn make_request(&self, resource_path: String, method: Method) -> Request {
        Request::new(method, String::from(self.base_url))
    }

    pub fn get(&self, resource_path: String) {
        self.make_request(resource_path, Method::GET);
    }
}