use {
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
}

impl ProxyClient {
    pub fn new() -> ProxyClient {
        ProxyClient {
            http_client: Client::new(),
        }
    }
    pub fn request(&self, req: Request<Body>) -> ResponseFuture {
        debug!("Request in proxy client {:?}", req);
        return self.http_client.request(req);
    }
}
