
use std::collections::HashMap;
use super::methods::Method;
use std::net::{
    IpAddr,
    TcpStream,
};

pub struct Request {
    method: Method,
    path: String,
    body: String,
    headers: HashMap<String, String>
}

impl Request {
    pub fn new(method: Method, host: String) -> Request {
        let mut req = Request {
            method,
            path: String::from("/"),
            body: String::from(""),
            headers: HashMap::new()
        };
        req.headers.insert(String::from("Host"), host);
        req
    }

    fn format_message(&self) -> String {
        let mut msg = format!("{} {} {}\r\n", self.method.value(), self.path , "HTTP1/1");

        for (k,v) in self.headers {
            msg = msg.add(format!("{}:{}\r\n", k, v).as_str())
        }
        msg = msg.add(self.body.as_str());
        msg
    }

    pub fn send(&self) {
        //TcpStream::connect()
    }
}


#[cfg(test)]
mod test {
    use super::Request;
    use super::super::methods;
    #[test]
    fn request_can_instantiate() {
        let m = methods::GET;
        let host = String::from("http://www.foo.com");
        let req = Request::new(m, host);
    }
}

