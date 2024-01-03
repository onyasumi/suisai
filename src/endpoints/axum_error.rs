use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};


/// Wrapper for anyhow::Error that also comes with a axum::http::StatusCode
pub struct AxumError(anyhow::Error, StatusCode);


/// Trait that allows for easy conversion from a generic error to an AxumError
pub trait IntoAxumError<T, E> {

    /// Return Result<T, AxumError> from Result<T, E> using a given HTTP error code
    fn or_error(self, err_code: StatusCode) -> Result<T, AxumError>;

    /// Return Result<T, AxumError> from Result<T, E> using a default 500 error code
    fn or_500(self) -> Result<T, AxumError>;

}

impl<T, E> IntoAxumError<T, E> for Result<T, E> where E: Into<anyhow::Error> {

    fn or_error(self, err_code: StatusCode) -> Result<T, AxumError> {
        match self {
            Ok(val) => Ok(val),
            Err(err) => Err(AxumError(err.into(), err_code))
        }
    }

    fn or_500(self) -> Result<T, AxumError> {
        self.or_error(StatusCode::INTERNAL_SERVER_ERROR)
    }

}


/// Make AxumError cast-able into an Axum response
impl IntoResponse for AxumError {
    fn into_response(self) -> Response {
        (self.1, self.0.to_string()).into_response()
    }
}


/// Enables ? on return type AxumError with default err code 500
impl<E> From<E> for AxumError where E: Into<anyhow::Error> {
    fn from(err: E) -> Self {
        Self(err.into(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}