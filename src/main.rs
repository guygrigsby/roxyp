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

static FS: &[u8] = b"fileserver";

async fn proxy_req<T: Proxy>(proxy: T, req: Request<Body>) -> Result<Response<Body>, Error> {
    debug!("Request {:?}", req);

    let res = proxy.request(req).await?;

    debug!("Response {:?}", res);

    Ok(res)
}

async fn fs_req(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from(FS)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    pretty_env_logger::init();

    let proxy_addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT").map_or(3000, |v| v.parse().unwrap()),
    ));
    let fs_addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("FS_PORT").map_or(8080, |v| v.parse().unwrap()),
    ));

    let upstream = match env::var("UPSTREAM") {
        Ok(u) => u,
        Err(_) => args
            .get(1)
            .unwrap_or(&String::from("localhost:8080"))
            .to_string(),
    };

    info!(
        "Starting Server on {} with upstream {}",
        proxy_addr, &upstream
    );

    let client: FixedUpstream = proxy::FixedUpstream::new(upstream);

    // Call our `run_server` function, which returns a future.
    // As with every `async fn`, for `run_server` to do anything,
    // the returned future needs to be run using `await`;
    let proxy_srv = Server::bind(&proxy_addr)
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
    let fs_srv = Server::bind(&fs_addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(fs_req))
    }));
    tokio::spawn(async move { fs_srv.await });

    println!(
        "Listening on (proxy)http://{} and (fs)http://{}",
        proxy_addr, fs_addr
    );

    if let Err(e) = proxy_srv.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
