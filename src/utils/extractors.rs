use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::Request,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::utils::error::Error;

pub struct JsonExtractor<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for JsonExtractor<T>
where
    axum::Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = Error;
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Error> {
        let (parts, body) = req.into_parts();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => match value.validate() {
                Ok(_) => Ok(JsonExtractor(value.0)),
                Err(errors) => Err(Error::ValidationError(errors)),
            },

            Err(rejection) => Err(Error::JsonError(rejection)),
        }
    }
}
