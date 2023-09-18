use std::{
  fs::File,
  io::BufReader,
  path::{Path, PathBuf},
  pin::Pin,
  sync::Arc,
};

use tokio_rustls::{
  rustls::{Certificate, PrivateKey, ServerConfig},
  TlsAcceptor,
};

// use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub fn rustls_server_config(key: impl AsRef<Path>, cert: impl AsRef<Path>) -> Arc<ServerConfig> {
  let mut key_reader = BufReader::new(File::open(key).unwrap());
  let mut cert_reader = BufReader::new(File::open(cert).unwrap());

  let key = PrivateKey(pkcs8_private_keys(&mut key_reader).unwrap().remove(0));
  let certs = certs(&mut cert_reader)
      .unwrap()
      .into_iter()
      .map(Certificate)
      .collect();

  let mut config = ServerConfig::builder()
      .with_safe_defaults()
      .with_no_client_auth()
      .with_single_cert(certs, key)
      .expect("bad certificate/key");

  config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

  Arc::new(config)
}