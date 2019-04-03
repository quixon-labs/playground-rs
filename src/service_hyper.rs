use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use quixutils::prelude::*;
use std::net::SocketAddr;
use tokio::prelude::*;

pub(crate) fn run(opts: &super::Options) -> ResultAs<()> {
    let addr: SocketAddr = opts.address.parse()?;
    let serv = hyper::Server::try_bind(&addr)?;

    let service = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        service_fn(move |req: Request<Body>| handle(remote_addr, req))
    });

    let future = serv
        .tcp_keepalive(Some(std::time::Duration::from_secs(60)))
        .serve(service);

    info!("starting");

    tokio::run(future.map_err(|e| error!("{:?}", e)));
    Ok(())
}

fn handle(
    remote_addr: SocketAddr,
    req: Request<Body>,
) -> Box<Future<Item = Response<Body>, Error = failure::Error> + Send> {
    match req.uri().path() {
        "/" => {
            trace!("route: root");
            let path = crate::utils::TEST_FILE;
            let f = tokio::fs::read(path)
                .map_err(|e| failure::format_err!("error: {:?}", e))
                .and_then(|data| {
                    trace!("file data retrieved");
                    let body = Body::from(data);
                    let res = Response::builder().body(body);
                    res.into_future()
                        .map_err(|e| failure::format_err!("error: {:?}", e))
                });
            Box::new(f)
        }
        _ => {
            trace!("route: fallback");
            let res = Response::builder().body(Body::from(format!("Hello from {}", remote_addr)));
            let f = res
                .into_future()
                .map_err(|e| failure::format_err!("error: {:?}", e));
            Box::new(f)
        }
    }
}
