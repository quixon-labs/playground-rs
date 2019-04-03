#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

use structopt::StructOpt;

mod service_actix;
mod service_hyper;
mod service_tide;
mod service_warp;
mod utils;

fn main() -> Result<(), exitfailure::ExitFailure> {
    let opts = Options::from_args();
    quixutils::logger::init_with_verbosity(opts.verbosity);

    match opts.service_type {
        ServiceType::Hyper => service_hyper::run(&opts)?,
        ServiceType::Actix => service_actix::run(&opts)?,
        ServiceType::Warp => service_warp::run(&opts)?,
        ServiceType::Tide => service_tide::run(&opts)?,
    }
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt()]
pub(crate) struct Options {
    /// Verbosity
    #[structopt(short, long, parse(from_occurrences), default_value = "1")]
    pub verbosity: u8,

    /// Address
    #[structopt(short, long, default_value = "0.0.0.0:14900")]
    pub address: String,

    /// Service type
    #[structopt(
        short = "t",
        long,
        default_value = "hyper",
        raw(
            possible_values = "&ServiceType::variants()",
            case_insensitive = "true"
        )
    )]
    pub service_type: ServiceType,

    /// TLS cert path
    #[structopt(
        short = "c",
        long,
        default_value = "/srv/letsencrypt/live/quixon.com/fullchain.pem"
    )]
    pub tls_cert_path: String,

    /// TLS key path
    #[structopt(
        short = "k",
        long,
        default_value = "/srv/letsencrypt/live/quixon.com/privkey.pem"
    )]
    pub tls_key_path: String,

    /// TLS key, cert in PKCS12 format
    #[structopt(
        short = "x",
        long,
        default_value = "/srv/letsencrypt/live/quixon.com/cert.pfx"
    )]
    pub tls_pkcs12: String,
}

arg_enum! {
    #[derive(Debug, Clone, Copy)]
    pub enum ServiceType {
        Actix,
        Hyper,
        Warp,
        Tide
    }
}
