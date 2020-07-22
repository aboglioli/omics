use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
  Ok(Response::new("
      <html>
        <head>
          <title>Omics</title>
          <style>
            html, body {
              padding: 0;
              margin: 0;
              display: flex;
              align-items: center;
              justify-content: center;
              background-color: #1b1b1b;
            }
            .text {
              font-size: 3rem;
              padding: 3rem;
              display: flex;
              justify-content: center;
              align-items: center;
              background-color: #1f1f1f;
              color: #942121;
              border-radius: 3px;
            }
          </style>
        </head>
        <body>
          <b class=\"text\">
            Omics
          </b>
        </body>
      </html>
  ".into()))
}

#[tokio::main]
async fn main() {
    let port = 3000;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on port {}", port);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
