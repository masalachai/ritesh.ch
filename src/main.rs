use std::env;
use std::net::IpAddr;

use actix_web::{web, App, HttpServer, Result, error::Error, middleware::Logger, HttpRequest, HttpResponse, error::ErrorInternalServerError};
use handlebars::Handlebars;
use serde_json::json;
use serde::{Serialize, Deserialize};
use actix_files as fs;
use recaptcha;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/icons/favicon.ico")?)
}

async fn pdf(req: HttpRequest) -> Result<fs::NamedFile> {
    let filename: String = req.match_info().get("name").unwrap().parse().unwrap();
    let path = format!("./static/files/{}.pdf", filename);
    Ok(fs::NamedFile::open(path)?.set_content_type(mime::APPLICATION_PDF).disable_content_disposition())
}

#[derive(Debug, Deserialize)]
struct ContactForm {
    #[serde(rename = "g-recaptcha-response")]
    g_recaptcha_response: String,
    sender_email: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ContactResponse {
    status: u32,
    message: String,
}

async fn contact(req: HttpRequest, form: web::Form<ContactForm>) -> Result<HttpResponse, Error> {
    println!("{:?}", form);
    let captcha_secret = env::var("CAPTCHA_SECRET").unwrap();
    let smtp_host = env::var("SMTP_HOST").unwrap();
    let smtp_username = env::var("SMTP_USERNAME").unwrap();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap();
    let smtp_recipient = env::var("SMTP_RECIPIENT").unwrap();
    //let smtp_recipient_name = env::var("SMTP_RECIPIENT_NAME").unwrap();

    let connection_info = req.connection_info();

    let ip_addr_string = connection_info.realip_remote_addr().unwrap().trim_end_matches(|c: char| c.is_numeric()).trim_end_matches(':');

    let ip_addr: IpAddr = ip_addr_string.parse().unwrap();

    println!("IP addr: {:?}", ip_addr);

    let response = recaptcha::verify(&captcha_secret, &form.g_recaptcha_response, Some(&ip_addr)).await;

    if response.is_ok() {
        let email = Message::builder()
            .from(form.sender_email.parse().unwrap())
            .reply_to(form.sender_email.parse().unwrap())
            .to(smtp_recipient.parse().unwrap())
            .subject("Message from the CV website")
            .body(String::from(&form.message))
            .unwrap();

        let creds = Credentials::new(smtp_username, smtp_password);

        let mailer = SmtpTransport::starttls_relay(&smtp_host)
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => Ok(HttpResponse::Ok().json(
                        ContactResponse {
                            status: 200,
                            message: String::from("Thanks for contacting me :)")
                        }
                    )),
            Err(_e) => Ok(HttpResponse::Ok().json(
                        ContactResponse {
                            status: 500,
                            message: String::from("Oops! Something went wrong when sending the email")
                        }
                    )),
        }
    } else {
        Ok(HttpResponse::Ok().json(
            ContactResponse {
                status: 403,
                message: String::from("Invalid Captcha")
            }
        ))
    }
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

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", web::get().to(index))
            .route("/contact", web::post().to(contact))
            .route("/files/{name}.pdf", web::get().to(pdf))
            .route("/favicon.ico", web::get().to(favicon))
            .service(fs::Files::new("/static", "./static").prefer_utf8(true))
    })
    .bind(bind)?
    .run()
    .await
}
