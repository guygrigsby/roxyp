use {
    hyper::{
        // Following functions are used by Hyper to handle a `Request`
        // and returning a `Response` in an asynchronous manner by using a Future
        service::{make_service_fn, service_fn},
        // Miscellaneous types from Hyper for working with HTTP.
        Body,
        //Client,
        //Error,
        Request,
        Response,
        Server,
    },
    std::env,
    std::net::SocketAddr,
};

async fn proxy_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Listening on http://{:?}", _req);
    // Always return successfully with a response containing a body with
    // a friendly greeting ;)
    Ok(Response::new(Body::from("hello, world!")))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // Set the address to run our socket on.
    // let my_int = my_string.parse::<i32>().unwrap();
    let port: u16 = env::var("PORT").ok().map_or(3000, |v| v.parse().unwrap());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Call our `run_server` function, which returns a future.
    // As with every `async fn`, for `run_server` to do anything,
    // the returned future needs to be run using `await`;
    run_server(addr).await;
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    // Create a server bound on the provided address
    let serve_future = Server::bind(&addr)
        // Serve requests using our `async proxy_req` function.
        // `serve` takes a closure which returns a type implementing the
        // `Service` trait. `service_fn` returns a value implementing the
        // `Service` trait, and accepts a closure which goes from request
        // to a future of the response.
        // nothing is passed to the closure because it's not anonymous '|_|'
        .serve(make_service_fn(move |_| async {
            {
                Ok::<_, hyper::Error>(service_fn(proxy_req))
            }
        }));

    // Wait for the server to complete serving or exit with an error.
    // If an error occurred, print it to stderr.
    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}
