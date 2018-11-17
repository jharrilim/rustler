pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT
}

impl Method {
    pub fn value(&self) -> String {
        match *self {
            Method::OPTIONS => String::from("OPTIONS"),
            Method::GET => String::from("GET"),
            Method::HEAD => String::from("HEAD"),
            Method::POST => String::from("POST"),
            Method::PUT => String::from("PUT"),
            Method::DELETE => String::from("DELETE"),
            Method::TRACE => String::from("TRACE"),
            Method::CONNECT => String::from("CONNECT")
        }
    }
}
