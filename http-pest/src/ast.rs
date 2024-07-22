pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

pub enum Scheme {
    Http,
    Https,
}

pub enum Version {
    Http1_1,
}

pub struct Url {
    scheme: Scheme,
    host: String,
    port: Option<u16>,
    path: String,
    params: Vec<Param>,
}

pub struct Param {
    name: String,
    value: String,
}

pub struct Header {
    name: String,
    value: String,
}

pub struct Body(String);

pub struct Request {
    method: Method,
    url: Url,
    headers: Vec<Header>,
    body: Option<Body>,
}
