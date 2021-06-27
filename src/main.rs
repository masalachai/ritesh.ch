use std::env;

use actix_web::{web, App, HttpServer, Result, error::Error, HttpRequest, HttpResponse, error::ErrorInternalServerError};
use handlebars::Handlebars;
use serde_json::json;
use actix_files as fs;

async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/icons/favicon.ico")?)
}

async fn pdf(req: HttpRequest) -> Result<fs::NamedFile> {
    let filename :String = req.match_info().get("name").unwrap().parse().unwrap();
    let path = format!("./static/files/{}.pdf", filename);
    Ok(fs::NamedFile::open(path)?.set_content_type(mime::APPLICATION_PDF).disable_content_disposition())
}

async fn index() -> Result<HttpResponse, Error> {
    let json = &json!({
        "name": env::var("MY_NAME").unwrap(),
        "phone": env::var("MY_PHONE").unwrap(),
        "cv_link": env::var("CV_LINK").unwrap(),
        "captcha_sitekey": env::var("CAPTCHA_SITEKEY").unwrap(),
    });

    let mut hb = Handlebars::new();

    hb.register_template_file("index", "./templates/index.hbs").unwrap_or_else (|err| {
        ErrorInternalServerError(err);
    });

    match hb.render("index", json) {
        Ok(contents) => Ok(HttpResponse::Ok().content_type("text/html").body(contents)),
        Err(err) => Err(ErrorInternalServerError(err))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = match env::var("SERVICE_PORT") {
        Ok(port) => format!("0.0.0.0:{}", port),
        Err(_) => String::from("0.0.0.0:8080")
    };

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/")
                    .route(web::get().to(index))
            )
            .route("/files/{name}.pdf", web::get().to(pdf))
            .route("/favicon.ico", web::get().to(favicon))
            .service(fs::Files::new("/static", "./static").prefer_utf8(true))
    })
    .bind(bind)?
    .run()
    .await
}
