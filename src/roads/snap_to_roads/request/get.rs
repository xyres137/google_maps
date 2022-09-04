use backoff::Error::{Permanent, Transient};
use backoff::ExponentialBackoff;
use backoff::future::retry;
use crate::request_rate::api::Api;
use crate::roads::error::Error;
use crate::roads::snap_to_roads::{SERVICE_URL, request::Request, response::Response};

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Performs the HTTP get request and returns the response to the caller.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    #[tracing::instrument(level = "debug", name = "Google Maps Snap-To-Roads", skip(self))]
    pub async fn get(&mut self) -> Result<Response, Error> {

        // Build the URL stem for the HTTP get request:
        let mut url = format!("{}/?", SERVICE_URL);

        match &self.query {
            // If query string built, append it to the URL stem.
            Some(query) => url.push_str(query.as_ref()),
            // If query string not built, return an error.
            None => return Err(Error::QueryNotBuilt),
        } // match

        // Observe any rate limiting before executing request:
        self.client_settings.rate_limit.limit_apis(vec![&Api::All, &Api::Roads])
            .await;

        // Emit debug message so client can monitor activity:
        tracing::info!("Making HTTP GET request to Google Maps Roads API: `{url}`");

        // Retries the get request until successful, an error ineligible for
        // retries is returned, or we have reached the maximum retries. Note:
        // errors wrapped in `Transient()` will retried by the `backoff` crate
        // while errors wrapped in `Permanent()` will exit the retry loop.
        retry(ExponentialBackoff::default(), || async {

            // Query the Google Cloud Maps Platform using using an HTTP get
            // request, and return result to caller:
            let response: Result<reqwest::Response, reqwest::Error> =
                match &self.client_settings.reqwest_client {
                    Some(reqwest_client) =>
                        match reqwest_client.get(&*url).build() {
                            Ok(request) => reqwest_client.execute(request).await,
                            Err(error) => Err(error),
                        }, // Some
                    None => reqwest::get(&*url).await,
                }; // match

            // Check response from the HTTP client:
            match response {
                Ok(response) => {
                    // HTTP client was successful getting a response from the
                    // server. Check the HTTP status code:
                    if response.status().is_success() {
                        // If the HTTP GET request was successful, get the
                        // response text:
                        let text = &response.text().await;
                        match text {
                            Ok(text) => {
                                match serde_json::from_str::<Response>(text) {
                                    Ok(deserialized) => {
                                        // Google API returned an error. This
                                        // indicates an issue with the request.
                                        // In most cases, retrying will not
                                        // help:
                                        if let Some(error) = deserialized.error {
                                            let error = Error::GoogleMapsService(
                                                error.status.to_owned(),
                                                Some(error.message),
                                            );
                                            tracing::error!("{}", error);
                                            Err(Permanent(error))
                                        // If the response JSON was successfully
                                        // parsed, check the Google API status
                                        // before returning it to the caller:
                                        } else {
                                            // If Google's response did not
                                            // contain an `ErrorResponse`
                                            // struct, return the struct
                                            // deserialized from JSON:
                                            Ok(deserialized)
                                        } // if
                                    }, // Ok(deserialized)
                                    Err(error) => {
                                        tracing::error!("JSON parsing error: {}", error);
                                        Err(Permanent(Error::SerdeJson(error)))
                                    }, // Err
                                } // match
                            }, // Ok(text)
                            Err(error) => {
                                tracing::error!("HTTP client returned: {}", error);
                                Err(Permanent(Error::ReqwestMessage(error.to_string())))
                            }, // Err
                        } // match
                    // We got a response from the server but it was not OK.
                    // Only HTTP "500 Server Errors", and HTTP "429 Too Many
                    // Requests" are eligible for retries.
                    } else if response.status().is_server_error() || response.status() == 429 {
                        tracing::warn!("HTTP client returned: {}", response.status());
                        Err(Transient { err: Error::HttpUnsuccessful(response.status().to_string()), retry_after: None })
                    // Not a 500 Server Error or "429 Too Many Requests" error.
                    // The error is permanent, do not retry:
                    } else {
                        tracing::error!("HTTP client returned: {}", response.status());
                        Err(Permanent(Error::HttpUnsuccessful(response.status().to_string())))
                    } // if
                } // case
                // HTTP client did not get a response from the server. Retry:
                Err(error) => {
                    tracing::warn!("HTTP client returned: {}", error);
                    Err(Transient { err: Error::Reqwest(error), retry_after: None })
                } // case
            } // match

        }).await

    } // fn

} // impl