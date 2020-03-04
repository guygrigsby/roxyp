use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Error, Request, Response, Server,
    },
    log::{debug, info},
    proxy::FixedUpstream,
    proxy::Proxy,
    std::convert::Infallible,
    std::env,
    std::net::SocketAddr,
};

mod cache;
mod proxy;

async fn proxy_req<T: Proxy>(proxy: T, req: Request<Body>) -> Result<Response<Body>, Error> {
    debug!("Request {:?}", req);

    let res = proxy.request(req).await?;

    debug!("Response {:?}", res);

    Ok(res)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    pretty_env_logger::init();

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT").map_or(3000, |v| v.parse().unwrap()),
    ));

    let upstream = match env::var("UPSTREAM") {
        Ok(u) => u,
        Err(_) => args
            .get(1)
            .unwrap_or(&String::from("localhost:8080"))
            .to_string(),
    };

    info!("Starting Server on {} with upstream {}", addr, &upstream);

    let client: FixedUpstream = proxy::FixedUpstream::new(upstream);

    // Call our `run_server` function, which returns a future.
    // As with every `async fn`, for `run_server` to do anything,
    // the returned future needs to be run using `await`;
    let serve_future = Server::bind(&addr)
        // Serve requests using our `async proxy_req` function.
        // `serve` takes a closure which returns a type implementing the
        // `Service` trait. `service_fn` returns a value implementing the
        // `Service` trait, and accepts a closure which goes from request
        // to a future of the response.
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
