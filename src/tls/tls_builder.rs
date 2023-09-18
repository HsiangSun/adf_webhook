use std::sync::Arc;

use tokio_rustls::{
  rustls,
  rustls::{Certificate, PrivateKey, ServerConfig},
  TlsAcceptor,
};
use rustls_pemfile::Item;
use rustls::server::ClientCertVerifier;

async fn build_rustls_server_config(cert: &str, key: &str, ca: Option<&str>) -> Arc<ServerConfig> {
  let cert = tokio::fs::read(cert).await.unwrap();
  let key = tokio::fs::read(key).await.unwrap();

  // get pem from file
  let cert = rustls_pemfile::certs(&mut cert.as_ref()).unwrap();
  let key = match rustls_pemfile::read_one(&mut key.as_ref()).unwrap() {
      Some(Item::RSAKey(key)) | Some(Item::PKCS8Key(key)) => key,
      // rustls only support PKCS8, does not support ECC private key
      _ => panic!("private key invalid or not supported"),
  };
  let cert = cert.into_iter().map(rustls::Certificate).collect();
  let key = rustls::PrivateKey(key);

  let config_builder = rustls::ServerConfig::builder().with_safe_defaults();

  let mut server_config = match ca {
      None => {
          tracing::info!("mTLS disabled");
          config_builder
              .with_no_client_auth()
              .with_single_cert(cert, key)
              .expect("bad certificate/key")
      },
      Some(ca) => {
          tracing::info!("mTLS enabled, ca cert path={}", ca);
          let ca = tokio::fs::read(ca).await.unwrap();
          if let Some(Item::X509Certificate(ca)) =
              rustls_pemfile::read_one(&mut ca.as_ref()).unwrap()
          {
              let mut root_cert_store = rustls::RootCertStore::empty();
              root_cert_store
                  .add(&rustls::Certificate(ca))
                  .expect("bad ca cert");
              config_builder
                  .with_client_cert_verifier(WebPkiClientVerifier::no_client_auth())
                  .with_single_cert(cert, key)
                  .expect("bad certificate/key")
          } else {
              panic!("invalid root ca cert")
          }
      }
  };

  server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
  Arc::new(server_config)
}