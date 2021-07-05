use std::{env, time::Duration};

use handlebars::Handlebars;
use serde_json::json;
use futures_core::future::BoxFuture;
use futures_util::FutureExt;
use mime;
use northstar::{Server, Request, Response, Body, GEMINI_PORT, GEMINI_MIME};
use anyhow::{anyhow};
use tokio;

fn serve_pdf(_: Request) -> BoxFuture<'static, anyhow::Result<Response>> {
    async move {
        let response = northstar::util::serve_file("./static/files/ritesh-chitlangi-2018.pdf", &mime::APPLICATION_PDF).await?;

        Ok(response)
    }
    .boxed()
}

fn favicon(_: Request) -> BoxFuture<'static, anyhow::Result<Response>> {
    async move {
        Ok(Response::success(&GEMINI_MIME, Body::from("\u{1F496}\r\n")))
    }
    .boxed()
}

fn index(request: Request) -> BoxFuture<'static, anyhow::Result<Response>> {


    async move {
        if let 0 = request.trailing_segments().len() {
            let json = &json!({
                "name": env::var("MY_NAME").unwrap(),
            });

            let mut hb = Handlebars::new();

            hb.register_template_file("index", "./templates/index.gmi.hbs").unwrap_or_else (|err| {
                anyhow!("{}", err);
            });

            match hb.render("index", json) {
                Ok(contents) => Ok(Response::success(&GEMINI_MIME, Body::from(contents))),
                Err(err) => Err(anyhow!("{}", err))
            }
        } else {
            Ok(Response::not_found())
        }
    }
    .boxed()
}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    env_logger::init();

    let gemini_port = match env::var("GEMINI_PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => GEMINI_PORT
    };

    Server::bind(("0.0.0.0", gemini_port))
        .add_route("/", index)
        .add_route("/index.gmi", index)
        .add_route("/favicon.txt", favicon)
        .add_route("/resume.pdf", serve_pdf)
        .set_timeout(Duration::from_secs(10))
        .serve()
        .await
}