use actix_web::{middleware, web, App, HttpServer};
use okapi_rewrite::routes::adventures::genadventures;
use okapi_rewrite::routes::chess::genchess;
use okapi_rewrite::routes::imageops::*;
use okapi_rewrite::routes::index::index;
use okapi_rewrite::routes::overlay::genoverlay;
use okapi_rewrite::routes::profile::genprofile;
use std::env::set_var;
use std::io::Result as IoResult;

#[actix_web::main]
async fn main() -> IoResult<()> {
    set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(index)
            .service(genadventures)
            .service(genchess)
            .service(genoverlay)
            .service(genprofile)
            .service(pixelate)
            .service(invert_endpoint)
            .service(edges_endpoint)
            .service(oil_endpoint)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
