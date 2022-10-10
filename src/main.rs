
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::http::HeaderValue;
use hyper::{Body, Request, Response, Server, HeaderMap};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); 
    let port = std::env::var("PING_LISTEN_PORT");

    let port = match port {
        Ok(port) => port.parse().unwrap(),
        Err(error) => panic!("Error while trying to get your PING_LISTEN_PORT environment variable, please do not forget to set it before running your server : {:?}", error),
    };
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc = make_service_fn(|_connß| async {
        Ok::<_, Infallible>(service_fn(router))
    });

    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/ping") => {
            response.headers_mut().insert("Content-Type", "application/json".parse().unwrap());
            assert!(!response.headers().is_empty());
            *response.body_mut() = Body::from(convert(req.headers()).to_string());

        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
    Ok(response)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

fn convert(headers: &HeaderMap<HeaderValue>) -> serde_json::Value {
    format!("{:?}", headers).into()
}

