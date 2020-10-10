use actix_web::{
    error::{Error, InternalError, JsonPayloadError},
    middleware, web, App, HttpRequest, HttpResponse, HttpServer,
};
use std::env::{set_var, var};
use std::io::Result as IoResult;

pub mod constants;
pub mod encoder;
pub mod font;
pub mod loaders;
pub mod proxy;
pub mod routes;

/// Return either a 400 or 415, and include the error message from serde
/// in the response body
fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let detail = format!(
        "{{\"status\": \"error\", \"detail\": {:?}}}",
        err.to_string()
    );
    let response = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType()
            .content_type("application/json")
            .body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity()
                .content_type("application/json")
                .body(detail)
        }
        _ => HttpResponse::BadRequest()
            .content_type("application/json")
            .body(detail),
    };
    InternalError::from_response(err, response).into()
}

#[actix_web::main]
async fn main() -> IoResult<()> {
    set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let listen_address = match var("PORT") {
        Ok(p) => format!("0.0.0.0:{}", p),
        Err(_) => "0.0.0.0:3000".to_string(),
    };

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(
                web::JsonConfig::default()
                    .limit(65536)
                    .error_handler(json_error_handler),
            )
            .service(routes::index::index)
            .service(routes::adventures::genadventures)
            .service(routes::chess::genchess)
            .service(routes::overlay::genoverlay)
            .service(routes::profile::genprofile)
            .service(routes::imageops::pixelate)
            .service(routes::imageops::invert_endpoint)
            .service(routes::imageops::edges_endpoint)
            .service(routes::imageops::oil_endpoint)
    })
    .bind(listen_address)?
    .run()
    .await
}
