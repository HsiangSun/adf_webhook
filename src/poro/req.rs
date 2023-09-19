use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ReqData {
  pub table_name: String,
}