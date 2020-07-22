use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("¡Omics!".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on port {}", 3000);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
