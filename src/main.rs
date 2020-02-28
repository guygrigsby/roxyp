use {
    http::uri::Scheme,
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

fn create_up_req(req: Request<Body>) -> Request<Body> {
    debug!("Creating Upstream Request {:?}", req);
    let (mut parts, body) = req.into_parts();

    parts.uri = Uri::builder()
        .scheme(parts.uri.scheme().unwrap_or_else(|| &Scheme::HTTP).clone())
        .authority(
            parts
                .headers
                .get(HOST)
                .expect("Missing Host header")
                .to_str()
                .expect("failed to parse Host header"),
        )
        .path_and_query(
            parts
                .uri
                .path_and_query()
                .expect("Cannot get path and query from original request")
                .clone(),
        )
        .build()
        .expect("Failed to build upstream URI");

    return Request::from_parts(parts, body);
}

async fn proxy_req(client: ProxyClient, req: Request<Body>) -> Result<Response<Body>, Error> {
    debug!("Request {:?}", req);

    let res = client.request(create_up_req(req)).await?;

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

    let client = ProxyClient::new();

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
