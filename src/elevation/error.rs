//! Elevation API error types and error messages.

use crate::elevation::response::status::Status;

/// Errors that may be produced by the Google Maps Elevation API client.
#[derive(Debug)]
pub enum Error {
    /// A sampled_path_request() method cannot be used when postional_request()
    /// has been set.
    EitherPositionalOrSampledPath,
    /// Google Maps Elevation API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsElevationServer(Status, Option<String>),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Elevation API server.
    QueryNotBuilt,
    /// The request must be validated before a query string may be built.
    RequestNotValidated,
    /// The dependency library Reqwest generated an error.
    Reqwest(reqwest::Error),
    /// The dependency library Serde JSON generated an error.
    SerdeJson(serde_json::error::Error),
} // enum

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::EitherPositionalOrSampledPath => write!(f, "Google Maps Elevation API client library: \
                A sampled_path_request() method cannot be used when postional_request() has been set. \
                Try again with only a positional request or only a sampled path request."),
            Error::GoogleMapsElevationServer(status, error_message) => match error_message {
                // If the Google Maps Elevation API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Elevation API server: {}", error_message),
                // If the Google Maps Elevation API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f, "Google Maps Elevation API server: \
                        Invalid request. \
                        The request was malformed."),
                    Status::Ok => write!(f, "Google Maps Elevation server: \
                        Ok. \
                        The request was successful."),
                    Status::OverDailyLimit => write!(f, "Google Maps Elevation API server: \
                        Over daily limit. \
                        Usage cap has been exceeded, API key is invalid, billing has not been enabled, or method of payment is no longer valid."),
                    Status::OverQueryLimit => write!(f, "Google Maps Elevation API server: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f, "Google Maps Elevation API server: \
                        Request denied \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f, "Google Maps Elevation API server: \
                        Unknown error."),
                } // match
            }, // match
            Error::RequestNotValidated => write!(f, "Google Maps Elevation API client library: \
                The request must be validated before a query string may be built. \
                Ensure the validate() method is called before build()."),
            Error::Reqwest(error) => write!(f, "Google Maps Elevation API client in the Reqwest library: {}", error),
            Error::SerdeJson(error) => write!(f, "Google Maps Elevation API client in the Serde JSON library: {}", error),
            Error::QueryNotBuilt => write!(f, "Google Maps Elevation API client library: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
        } // match
    } // fn
} // impl

impl std::error::Error for Error {
    /// If the cause for the error is in an underlying library (not this
    /// library but a library this one depends on), this trait unwraps the
    /// original source error. This trait converts a Google Maps Elevation API
    /// error type into the native error type of the underlying library.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::EitherPositionalOrSampledPath => None,
            Error::GoogleMapsElevationServer(_error, _message) => None,
            Error::RequestNotValidated => None,
            Error::Reqwest(error) => Some(error),
            Error::SerdeJson(error) => Some(error),
            Error::QueryNotBuilt => None,
        } // match
    } // fn
} // impl

impl From<reqwest::Error> for Error {
    /// This trait converts from an Reqwest error type (`reqwest::Error`) into a
    /// Google Maps Elevation API error type
    /// (`google_maps::elevation::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    } // fn
} // impl

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Elevation API error type
    /// (`google_maps::elevation::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    } // fn
} // impl