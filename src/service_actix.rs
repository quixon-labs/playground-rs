use actix::prelude::*;
use actix_web::http::StatusCode;
use actix_web::{App, HttpRequest, Responder};
use quixutils::prelude::*;

pub(crate) fn run(opts: &super::Options) -> ResultAs<()> {
    let sys = System::new("main");
    let sync_pool = SyncArbiter::start(4, || SyncExecutor {});

    let ctx = ReqContext { sync_pool };

    let serv = actix_web::server::new(move || create_app(&ctx));
    let addr = opts.address.clone();
    serv.bind(addr)?.start();
    sys.run();
    Ok(())
}

pub struct ReqContext {
    sync_pool: Addr<SyncExecutor>,
}

impl Clone for ReqContext {
    fn clone(&self) -> Self {
        ReqContext {
            sync_pool: self.sync_pool.clone(),
        }
    }
}

struct SyncExecutor {}

impl Actor for SyncExecutor {
    type Context = SyncContext<Self>;
}

fn create_app(context: &ReqContext) -> App<ReqContext> {
    use actix_web::http::NormalizePath;
    info!("creating actix app");

    App::with_state(context.clone())
        .middleware(actix_web::middleware::Logger::default())
        // .resource("/", |r| r.method(Method::GET).f(index))
        .resource("/{tail:.*}", |r| r.f(default_response))
        .default_resource(|r| {
            r.h(NormalizePath::new(
                true,
                true,
                StatusCode::PERMANENT_REDIRECT,
            ))
        })
}

fn default_response(req: &HttpRequest<ReqContext>) -> impl Responder {
    trace!("route: fallback");
    let socket_addr = req.peer_addr().unwrap();
    format!("Hello from {}!", socket_addr)
}
