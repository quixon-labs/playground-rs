use http_service::Body;
use quixutils::prelude::*;
use std::net::SocketAddr;
use tide::{configuration::ConfigurationBuilder, App, Response};

struct Services {}

impl Services {
    fn new() -> Services {
        Services {}
    }
}

impl Clone for Services {
    fn clone(&self) -> Services {
        Services {}
    }
}

pub(crate) fn run(opts: &super::Options) -> ResultAs<()> {
    let mut app = App::empty(Services::new());
    let addr: SocketAddr = opts.address.parse()?;

    let config = ConfigurationBuilder::default()
        .address(addr.ip().to_string())
        .port(addr.port())
        .finalize();

    app.config(config);
    app.at("/hello").get(hello);
    app.at("/file").get(file_handler);
    app.default_handler(async || "Hello there!");
    app.serve();
    Ok(())
}

async fn hello() -> &'static str {
    "Hello world!"
}

async fn file_handler() -> Response {
    use tokio::await;
    let file_data_res = await!(tokio::fs::read(crate::utils::TEST_FILE));
    if let Ok(res) = file_data_res {
        return tide::Response::new(Body::from(res));
    }
    let e = file_data_res.err();
    eprintln!("{:?}", e);
    http::Response::builder()
        .status(500)
        .body(Body::empty())
        .unwrap()
}
