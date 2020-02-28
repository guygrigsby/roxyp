//extern crate futures;
//use futures::future::*;
//use http::StatusCode;
use {
    http::uri::Authority,
    http::Uri,
    hyper::{
        header::HOST,
        // Following functions are used by Hyper to handle a `Request`
        // and returning a `Response` in an asynchronous manner by using a Future
        service::{make_service_fn, service_fn},
        // Miscellaneous types from Hyper for working with HTTP.
        Body,
        Error,
        Request,
        Response,
        Server,
    },
    log::{debug, info},
    proxy::ProxyClient,
    std::convert::Infallible,
    std::env,
    std::net::SocketAddr,
};

mod proxy;

async fn proxy_req(client: ProxyClient, req: Request<Body>) -> Result<Response<Body>, Error> {
    debug!("Request {:?}", req);

    let (mut parts, body) = req.into_parts();
    let upstream_host = parts.headers.get(HOST).unwrap();
    let uri = Uri::builder()
        .scheme("https")
        .authority(upstream_host.to_str().unwrap())
        .path_and_query("/")
        .build();
    parts.uri = uri.unwrap();

    let upstream_req = Request::from_parts(parts, body);

    let res = client.request(upstream_req).await?;

    debug!("Response {:?}", res);

    Ok(res)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT").ok().map_or(3000, |v| v.parse().unwrap()),
    ));

    let upstream = Authority::from_static("example.com");
    //let proxy_uri = "http://my-proxy:8080".parse().unwrap();
    let client = ProxyClient::new(upstream);

    info!("Starting Server on {}", addr);

    // Call our `run_server` function, which returns a future.
    // As with every `async fn`, for `run_server` to do anything,
    // the returned future needs to be run using `await`;
    let serve_future = Server::bind(&addr)
        // Serve requests using our `async proxy_req` function.
        // `serve` takes a closure which returns a type implementing the
        // `Service` trait. `service_fn` returns a value implementing the
        // `Service` trait, and accepts a closure which goes from request
        // to a future of the response.
        // nothing is passed to the closure because it's not anonymous '|_|'
        .serve(make_service_fn(move |_| {
            let client = client.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    proxy_req(client.clone(), req)
                }))
            }
        }));

    // Wait for the server to complete serving or exit with an error.
    // If an error occurred, print it to stderr.
    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}
