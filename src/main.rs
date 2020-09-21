use hyper::body::to_bytes;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

use tendermint_rpc::endpoint;

type IError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(handler))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handler(req: Request<Body>) -> Result<Response<Body>, IError> {
    let response = match (req.method(), req.uri().path()) {
        (&Method::POST, "/commit") => handle_commit(to_bytes(req.into_body()).await?),
        (&Method::POST, "/validators") => handle_validators(to_bytes(req.into_body()).await?),
        (method, path) => {
            let mut response =
                Response::new(format!("Wrong path or method: {} {}\n", method, path).into());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    };
    if let Err(err) = response.as_ref() {
        println!("Error: {}", err);
    }
    response
}

fn handle_commit(body: hyper::body::Bytes) -> Result<Response<Body>, IError> {
    let req: endpoint::commit::Request = serde_json::from_slice(&body)?;
    println!("Success commit: {:?}", req);
    Ok(Response::new("commit\n".into()))
}

fn handle_validators(body: hyper::body::Bytes) -> Result<Response<Body>, IError> {
    let req: endpoint::validators::Request = serde_json::from_slice(&body)?;
    println!("Success validators: {:?}", req);
    Ok(Response::new("commit\n".into()))
}
