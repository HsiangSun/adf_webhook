
use axum::{Json, Extension};
use axum::response::{Response, IntoResponse};
use axum::{
    http::StatusCode,
    routing::post,
    Router,
    http::Request,
};
use axum_server::accept::DefaultAcceptor;
use axum_server::tls_rustls::RustlsConfig;
use hyper::server::accept::Accept;
use hyper::server::conn::{AddrIncoming, Http};
use rustls::ServerConfig;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tower_http::add_extension::AddExtensionLayer;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport, Message};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use axum_auth::AuthBasic;
use std::net::SocketAddr;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use futures_util::future::poll_fn;
use tower::make::MakeService;

mod err;
mod poro;
mod tls;
use err::app_error::AppError;
use poro::config::AppConfig;
use poro::rsp::{AzureRsp, OutputObject};

use crate::tls::tls_config::rustls_server_config;


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


    let current_path = std::env::current_dir().expect("Can't get current path");

    println!("PATH:{}",current_path.to_string_lossy());

    let cert_path = PathBuf::from(current_path.clone()).join("certs").join("cert.pem");
    let key_path = PathBuf::from(current_path.clone()).join("certs").join("key.pem");

    println!("cert:{}",cert_path.to_string_lossy());
    println!("key:{}",key_path.to_string_lossy());


    //init tls
    let rustls_config = RustlsConfig::from_pem_file(
       cert_path,
       key_path
    ).await.unwrap();


    // let rustls_config = rustls_server_config(key_path,cert_path);


    // RustlsConfig::from_config(rustls_config);

    // let acceptor = TlsAcceptor::from(rustls_config);

    // let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();
    // let mut listener = AddrIncoming::from_listener(listener).unwrap();


    // let protocol = Arc::new(Http::new());


    let app = Router::new()
    .route("/webhook", post(webhook_handler))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(app_config));


    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));



    let rustls_config = ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth()
    .with_cert_resolver(todo!());




    tracing::info!("Server started, listening on {}",addr);

    let acceptor = DefaultAcceptor::new();

    axum_server::bind(addr)
    .acceptor(acceptor)
    .serve(app.into_make_service())
    .await
    .expect("Failed to start server");



    // let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    // // println!("Server started, listening on {addr}");
    // tracing::info!("Server started, listening on {}",addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .expect("Failed to start server");
}
