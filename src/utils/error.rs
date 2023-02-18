use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::error::Error as MongoError;
use serde_json::json;
use validator::ValidationErrors;

type OptionalValidationErrors = Option<Vec<ValidationError>>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unknown")]
    Mongo(#[from] MongoError),

    #[error("JsonError")]
    JsonError(#[from] JsonRejection),

    #[error("ValidationError")]
    ValidationError(#[from] ValidationErrors),

    #[error("Unknown")]
    Unknown,
}

#[derive(serde::Serialize)]
struct ErrorResponseMessage {
    kind: String,
    message: String,
    #[serde(rename = "validationError", skip_serializing_if = "Option::is_none")]
    validation_errors: OptionalValidationErrors,
}

#[derive(serde::Serialize)]
struct ValidationError {
    field: String,
    errors: Vec<String>,
}

impl Error {
    fn json(
        kind: String,
        message: String,
        validation_errors: OptionalValidationErrors,
    ) -> serde_json::Value {
        json!({
                "error": ErrorResponseMessage {
                    kind,
                    message,
                    validation_errors
                }
            }
        )
    }
    fn get_error(&self) -> (StatusCode, String, OptionalValidationErrors) {
        match self {
            Error::Mongo(error) => {
                let message = "Unknown error occurred.".to_string();
                tracing::error!("Mongodb error occurred: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, message, None)
            }

            Error::JsonError(rejection) => {
                let code = match rejection {
                    JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
                    JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (code, rejection.to_string(), None)
            }

            Error::ValidationError(errors) => {
                let message = "Validation error occurred on the listed fields.".to_string();
                let errors = errors.field_errors();
                let mut validation_errors: Vec<ValidationError> = Vec::new();

                for (field, error_messages) in errors {
                    validation_errors.push(ValidationError {
                        field: field.to_string(),
                        errors: error_messages
                            .into_iter()
                            .map(|message| match message.message.as_ref() {
                                Some(message) => message.to_string(),
                                None => "".to_string(),
                            })
                            .collect(),
                    })
                }
                (StatusCode::BAD_REQUEST, message, Some(validation_errors))
            }

            Error::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error occurred.".to_string(),
                None,
            ),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (code, message, validation_errors) = self.get_error();
        let body = Json(Error::json(self.to_string(), message, validation_errors));

        (code, body).into_response()
    }
}
