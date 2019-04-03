use quixutils::prelude::*;
use warp::Filter;

pub(crate) fn run(opts: &super::Options) -> ResultAs<()> {
    let hello_txt = warp::any().map(|| "Hello there!");
    let hello_file = warp::fs::file(crate::utils::TEST_FILE);
    let index = warp::get2().and(warp::path::end()).and(hello_file);

    info!("starting");

    use std::net::SocketAddr;
    let addr: SocketAddr = opts.address.parse()?;
    warp::serve(index.or(hello_txt))
        // .tls(opts.tls_cert_path.clone(), opts.tls_key_path.clone())
        .run(addr);
    Ok(())
}
