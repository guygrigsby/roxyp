use {
    http::uri::Authority,
    hyper::{
        client::{HttpConnector, ResponseFuture},
        //Error,
        Body,
        Client,
        Request,
    },
    log::debug,
};

#[derive(Clone, Debug)]
pub struct ProxyClient {
    http_client: Client<HttpConnector, Body>,
    upstream: Authority,
}

#[derive(Clone, Copy, Debug)]
struct Config {
    retry_canceled_requests: bool,
    set_host: bool,
}

impl ProxyClient {
    pub fn new(up: Authority) -> ProxyClient {
        ProxyClient {
            http_client: Client::new(),
            upstream: up,
        }
    }
    pub fn request(&self, req: Request<Body>) -> ResponseFuture {
        debug!("Request in proxy client {:?}", req);
        return self.http_client.request(req);
    }
}
