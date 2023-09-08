
use axum::{Json, Extension};
use axum::response::{Response, IntoResponse};
use axum::{
    http::StatusCode,
    routing::post,
    Router,
};
use tower_http::add_extension::AddExtensionLayer;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport, Message};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use axum_auth::AuthBasic;
use std::net::SocketAddr;

mod err;
mod poro;
use err::app_error::AppError;
use poro::config::AppConfig;
use poro::rsp::{AzureRsp, OutputObject};


async fn webhook_handler(
    AuthBasic((name, password)): AuthBasic,
    Extension(app_config): Extension<AppConfig>
) -> Result<Response, AppError> {

    match password {
        None => (),
        Some(pwd) => {
            if pwd != "seanisthebestmanintheworld" || name != "sean" {
                return Err(AppError::BasicAuthError)
            }
        }
    }
   
    let email = Message::builder()
    .from(format!("Data Factory <{}>",app_config.sender).parse().unwrap())
    //.reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
    // .to("Nayif <m.nayif@afun.com>".parse().unwrap())
    .to(format!("Nayif <{}>",app_config.receiver).parse().unwrap())
    .subject("Database sync complete")
    .header(ContentType::TEXT_PLAIN)
    .body(String::from("Hello Nayif, today's data have been synced. Have a good day :)"))
    .unwrap();

    let creds = Credentials::new(app_config.sender, app_config.smtp_token);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();

    // Send the email
    // mailer.send(&email).map_err(|err| AppError::SendEmailError(err.into()))?;
    mailer.send(&email)?;

    Ok(
        (
            StatusCode::OK,
            Json(AzureRsp {
                output:OutputObject{result:String::from("ok")},
                status_code: String::from("200"),
                error: None
            }),
          )
            .into_response()
    )

    // Ok(Response::Ok().body("Email sent successfully"))
}




// impl From<AppConfig> for AddExtensionLayer<AppConfig> {
//     fn from(config: AppConfig) -> Self {
//         AddExtensionLayer::new(config)
//     }
// }






#[tokio::main]
async fn main() {

    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // axum logs rejections from built-in extractors with the `axum::rejection`
            // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
            "adf_webhook=debug,tower_http=trace,axum::rejection=trace,lettre=info".into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    //init config
    let receiver = dotenv::var("receiverEmail").unwrap_or_else(|_|panic!("Not found .env file"));
    let sender = dotenv::var("senderEmail").unwrap_or_else(|_|panic!("Not found .env file"));
    let smtp_token = dotenv::var("SMTP_TOKEN").unwrap_or_else(|_|panic!("Not found .env file"));


    let app_config = AppConfig {
        sender,
        receiver,
        smtp_token,
    };

    tracing::info!("config ==> {:?}",app_config);

    let app = Router::new()
    .route("/webhook", post(webhook_handler))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(app_config));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    // println!("Server started, listening on {addr}");
    tracing::info!("Server started, listening on {}",addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
