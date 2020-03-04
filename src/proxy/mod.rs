use {
    http::uri::Scheme,
    http::Uri,
    hyper::{
        client::{HttpConnector, ResponseFuture},
        Body, Client, Request,
    },
    hyper_tls::HttpsConnector,
    log::debug,
};

#[derive(Clone, Debug)]
pub struct HostHeader {
    http_client: Client<HttpsConnector<HttpConnector>, Body>,
}

#[derive(Clone, Debug)]
pub struct FixedUpstream {
    http_client: Client<HttpsConnector<HttpConnector>, Body>,
    upstream: String,
}

pub trait Proxy {
    fn request(&self, req: Request<Body>) -> ResponseFuture;
}

impl Proxy for FixedUpstream {
    fn request(&self, req: Request<Body>) -> ResponseFuture {
        debug!("Request in proxy client {:?}", req);
        return self.http_client.request(self.create_up_req(req));
    }
}

impl FixedUpstream {
    pub fn new(upstream: String) -> Self {
        let https = HttpsConnector::new();

        FixedUpstream {
            http_client: Client::builder().build::<_, Body>(https),
            upstream: upstream,
        }
    }

    fn create_up_req(&self, req: Request<Body>) -> Request<Body> {
        debug!("Creating Upstream Request {:?}", req);
        let (mut parts, body) = req.into_parts();

        parts.uri = Uri::builder()
            .scheme(parts.uri.scheme().unwrap_or_else(|| &Scheme::HTTP).clone())
            .authority(self.upstream.as_str())
            .path_and_query(
                parts
                    .uri
                    .path_and_query()
                    .expect("Cannot get path and query from original request")
                    .clone(),
            )
            .build()
            .expect("Failed to build upstream URI");
        //TODO headers

        return Request::from_parts(parts, body);
    }
}
