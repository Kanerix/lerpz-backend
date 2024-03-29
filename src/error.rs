use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::{Deserialize, Serialize};
use tracing::{error, error_span};
use utoipa::ToSchema;

/// A type alias for `Result<T, HandlerError>`.
///
/// Used by handlers to return a response or an structured error.
pub type HandlerResult<T, D = ()> = std::result::Result<T, HandlerError<D>>;

/// Represents an error returned by a handler.
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct HandlerError<D = ()>
where
	D: Serialize + Send + Sync + ToSchema<'static>,
{
	/// HTTP status code for the error.
	#[serde(skip)]
	pub status_code: StatusCode,
	/// The error header.
	///
	/// The headline for the error. A short and precise
	/// explanation of the error that occurred.
	pub header: String,
	/// The error message.
	///
	/// A more detailed description of what wen't wrong
	/// or what to do next.
	pub message: String,
	/// Other details about the error.
	///
	/// Does not get send to the client if it's [`None`].
	/// The some variant should implement [`ToSchema`] so that
	/// an OpenAPI schema can be generated for the type.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[aliases(Detail = ToSchema)]
	pub detail: Option<D>,
	/// The actual error that occurred.
	///
	/// There might no be an actual error, in which case this
	/// field is [`None`]. Should never be exposed to the client
	/// for security reasons.
	///
	/// If this field contains an error, the log_id field should
	/// also be present, to identify the error in the logs.
	#[serde(skip)]
	pub inner: Option<anyhow::Error>,
	/// The log ID of the error send to the client.
	///
	/// This is automatically set when the response contains an error
	/// that should be tracked. This is not public, so that it never
	/// get's set manually.
	#[serde(skip_serializing_if = "Option::is_none")]
	log_id: Option<uuid::Uuid>,
}

impl<D> IntoResponse for HandlerError<D>
where
	D: Serialize + Send + Sync + ToSchema<'static>,
{
	/// Converts a [`HandlerError`] into a [`Response`].
	///
	/// This automatically logs errors to using [`tracing`]. This also
	/// sets the log_id field so that the error can be tracked.
	fn into_response(mut self) -> Response {
		if let Some(error) = self.inner.as_ref() {
			if let None = self.log_id {
				self.log_id = Some(uuid::Uuid::new_v4())
			};

			let HandlerError {
				ref header,
				ref message,
				ref log_id,
				..
			} = self;
			// The `log_id` is guaranteed to be set (above).
			let log_id = log_id.unwrap();

			if self.status_code.is_server_error() {
				error!(log_id = %log_id, server_error = %error, "An server error occurred");
			} else {
				error!(log_id = %log_id, client_error = %header, message = %message, "An client error occurred");
			}
		}

		(self.status_code, Json(self)).into_response()
	}
}

impl<E, D> From<E> for HandlerError<D>
where
	E: Into<anyhow::Error>,
	D: Serialize + Send + Sync + ToSchema<'static>,
{
	/// Turns any error into a [`HandlerError`].
	///
	/// This assumes that the error is an internal server error.
	/// This will also set the error in the `inner` field.
	fn from(value: E) -> Self {
		Self {
			status_code: StatusCode::INTERNAL_SERVER_ERROR,
			header: String::from("Something went wrong"),
			message: String::from("If this issue persists, please contact an administrator."),
			detail: None,
			inner: Some(value.into()),
			log_id: None, // This will be set in `into_response` if inner set.
		}
	}
}

impl<D> HandlerError<D>
where
	D: Serialize + Send + Sync + ToSchema<'static>,
{
	/// Create a new [`HandlerError`] with status code, header and message.
	///
	/// All optional fields are set to `None`. These can be set using functions
	/// found on the struct.
	pub fn new(
		status_code: StatusCode,
		header: impl Into<String>,
		message: impl Into<String>,
	) -> Self {
		Self {
			status_code,
			header: header.into(),
			message: message.into(),
			detail: None,
			inner: None,
			log_id: None,
		}
	}

	/// A generic response for someone that tries to access an authorized resource
	/// with proper authorization.
	pub fn unauthorized() -> Self {
		Self {
			status_code: StatusCode::UNAUTHORIZED,
			header: String::from("Unauthorized for resource"),
			message: String::from("You do not have permission to access this resource."),
			detail: None,
			inner: None,
			log_id: None,
		}
	}

	/// Adds a custom detail to the [`HandlerError`].
	pub fn with_detail<T>(mut self, detail: T) -> Self
	where
		T: Into<D>,
	{
		self.detail = Some(detail.into());
		self
	}

	/// Adds an error to the [`HandlerError`].
	pub fn with_error<E>(mut self, error: E) -> Self
	where
		E: Into<anyhow::Error>,
	{
		self.inner = Some(error.into());
		self
	}

	/// Sets the `log_id` field for the [`HandlerError`].
	///
	/// The `log_id` field is automatically set when the `inner` field is present and the
	/// `log_id` field is [`None`]. Changing this field might make it hard or impossible to
	/// track the error or in other ways, break how the error is logged.
	pub unsafe fn with_log_id<U>(&mut self, log_id: U)
	where
		U: Into<uuid::Uuid>,
	{
		self.log_id = Some(log_id.into());
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Serialize, ToSchema)]
	struct Detail {
		test_detail: String,
	}

	#[derive(thiserror::Error, Debug, ToSchema)]
	enum Error {
		#[error("This is a test error.")]
		RandomError,
	}

	#[test]
	fn test_internal_server_error() {
		let inner_error = Error::RandomError;
		let detail = Detail {
			test_detail: String::from("Test detail"),
		};

		let handler_error: HandlerError<Detail> = HandlerError::new(
			StatusCode::BAD_REQUEST,
			"This is a test",
			"This is a test handler error.",
		)
		.with_error(inner_error)
		.with_detail(detail);

		assert!(handler_error.inner.is_some());
		assert!(handler_error.detail.is_some());
		// `log_id` should only be set when turned into a response.
		assert!(handler_error.log_id.is_none());

		let response = handler_error.into_response();
		assert!(response.status().is_client_error());
	}

	#[test]
	fn test_any_error_to_handler_error() {
		let example_handler = || -> HandlerResult<i32, HandlerError> { Ok("abc".parse::<i32>()?) };
		assert!(example_handler().is_err());

		let error = example_handler().unwrap_err();
		assert!(error.status_code.is_server_error());
		assert!(error.inner.is_some());
		// `log_id` should only be set when turned into a response.
		assert!(error.log_id.is_none());
	}
}
