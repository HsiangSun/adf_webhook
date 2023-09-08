use anyhow::Error;
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use thiserror::Error;

use crate::poro::rsp::{AzureRsp, OutputObject,ErrorObject};

// Make our own error that wraps `anyhow::Error`.
#[derive(Debug, Error)]
pub enum  AppError{
  #[error("Authentication failed")]
  BasicAuthError,
  #[error("Send email failed: `{0}` ")]
  SendEmailError(Error),
  #[error("Other library error: `{0}` ")]
  OtherErr(Error)
}



// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
  fn into_response(self) -> Response {
      tracing::error!("ERROR ====> :{:?}",self);
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        // format!("Something went wrong: {}", self.0),
        Json(AzureRsp {
          output:OutputObject{result:String::from("failed")},
          status_code: String::from("500"),
          error: Some(ErrorObject{
            error_code: String::from("500"),
            message: String::from("send email failed")
          })
        })
      )
        .into_response()
  }
}


// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
// impl<E> From<E> for AppError
// where
//     E: Into<anyhow::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }

// covert lettre::transport::smtp::Error => AppErr
impl From<lettre::transport::smtp::Error> for AppError {
  fn from(err: lettre::transport::smtp::Error) -> Self {
    AppError::SendEmailError(anyhow::Error::msg(format!("SMTP error: {:?}", err)))
  }
}




