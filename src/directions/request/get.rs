use crate::directions::{
    error::Error as DirectionsError, request::Request as DirectionsRequest,
    response::status::Status as DirectionsStatus, response::Response as DirectionsResponse,
    OUTPUT_FORMAT, SERVICE_URL,
}; // use crate::directions
use crate::error::Error as GoogleMapsError;
use crate::request_rate::api::Api;
use backoff::future::retry;
use backoff::Error::{Permanent, Transient};
use backoff::ExponentialBackoff;

// -----------------------------------------------------------------------------

impl<'a> DirectionsRequest<'a> {
    /// Performs the HTTP get request and returns the response to the caller.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.

    #[tracing::instrument(level = "info", skip(self))]
    pub async fn get(&mut self) -> Result<DirectionsResponse, GoogleMapsError> {
        // Build the URL stem for the HTTP get request:
        let mut url = format!("{SERVICE_URL}/{OUTPUT_FORMAT}?");

        match &self.query {
            // If query string built, append it to the URL stem.
            Some(query) => url.push_str(query.as_ref()),
            // If query string not built, return an error.
            None => return Err(DirectionsError::QueryNotBuilt)?,
        } // match

        // Observe any rate limiting before executing request:
        tracing::info!("making HTTP GET request to Google Maps Directions API");

        self.client
            .rate_limit
            .limit_apis(vec![&Api::All, &Api::Directions])
            .await;

        tracing::debug!("{url}");

        // Retries the get request until successful, an error ineligible for
        // retries is returned, or we have reached the maximum retries. Note:
        // errors wrapped in `Transient()` will retried by the `backoff` crate
        // while errors wrapped in `Permanent()` will exit the retry loop.
        let response = retry(ExponentialBackoff::default(), || async {
            // Query the Google Cloud Maps Platform using using an HTTP get
            // request, and return result to caller:
            let response = self.client.get_request(&url).await;

            // Check response from the HTTP client:
            match response {
                Ok(response) => {
                    // HTTP client was successful getting a response from the
                    // server. Check the HTTP status code:
                    if response.status().is_success() {
                        // If the HTTP GET request was successful, get the
                        // response text:
                        let text = response.text().await;
                        let bytes = text.map(String::into_bytes);
                        match bytes {
                            Ok(mut bytes) => {
                                match simd_json::serde::from_slice::<DirectionsResponse>(&mut bytes) {
                                    Ok(deserialized) => {
                                        // If the response JSON was successfully
                                        // parsed, check the Google API status
                                        // before returning it to the caller:
                                        if deserialized.status == DirectionsStatus::Ok {
                                            // If Google's response was "Ok"
                                            // return the struct deserialized
                                            // from JSON:
                                            Ok(deserialized)
                                        // Google API returned an error. This
                                        // indicates an issue with the request.
                                        // In most cases, retrying will not
                                        // help:
                                        } else {
                                            let error = DirectionsError::GoogleMapsService(
                                                deserialized.status.clone(),
                                                deserialized.error_message,
                                            );
                                            // Check Google API response status
                                            // for error type:
                                            if deserialized.status == DirectionsStatus::UnknownError
                                            {
                                                // Only Google's "Unknown Error"
                                                // is eligible for retries:
                                                tracing::warn!("{}", error);
                                                Err(Transient {
                                                    err: error,
                                                    retry_after: None,
                                                })
                                            } else {
                                                // Not an "Unknown Error." The
                                                // error is permanent, do not
                                                // retry:
                                                tracing::error!("{}", error);
                                                Err(Permanent(error))
                                            } // if
                                        } // if
                                    } // Ok(deserialized)
                                    Err(error) => {
                                        tracing::error!("JSON parsing error: {}", error);
                                        Err(Permanent(DirectionsError::SimdJson(error)))
                                    } // Err
                                } // match
                            } // Ok(text)
                            Err(error) => {
                                tracing::error!("HTTP client returned: {}", error);
                                Err(Permanent(DirectionsError::ReqwestMessage(
                                    error.to_string(),
                                )))
                            } // Err
                        } // match
                    // We got a response from the server but it was not OK.
                    // Only HTTP "500 Server Errors", and HTTP "429 Too Many
                    // Requests" are eligible for retries.
                    } else if response.status().is_server_error() || response.status() == 429 {
                        tracing::warn!("HTTP client returned: {}", response.status());
                        Err(Transient {
                            err: DirectionsError::HttpUnsuccessful(response.status().to_string()),
                            retry_after: None,
                        })
                    // Not a 500 Server Error or "429 Too Many Requests" error.
                    // The error is permanent, do not retry:
                    } else {
                        tracing::error!("HTTP client returned: {}", response.status());
                        Err(Permanent(DirectionsError::HttpUnsuccessful(
                            response.status().to_string(),
                        )))
                    } // if
                } // case
                // HTTP client did not get a response from the server. Retry:
                Err(error) => {
                    tracing::warn!("HTTP client returned: {}", error);
                    Err(Transient {
                        err: DirectionsError::Reqwest(error),
                        retry_after: None,
                    })
                } // case
            } // match
        })
        .await?;

        // Return response to caller:
        Ok(response)
    } // fn
} // impl
