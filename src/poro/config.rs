#[derive(Debug, Clone)]
pub struct AppConfig {
  pub sender: String,
  pub receiver: String,
  pub smtp_token: String,
}