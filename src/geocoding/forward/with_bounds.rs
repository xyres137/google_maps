use crate::{
    bounds::Bounds,
    geocoding::forward::ForwardRequest,
}; // use

impl ForwardRequest {

    /// Specifies a bounding box for biasing results.
    ///
    /// # Arguments:
    ///
    /// * `bounds` - The bounding box of the viewport within which to bias
    /// geocode results more prominently. This parameter will only influence,
    /// not fully restrict, results from the geocoder.
    ///
    /// # Description
    ///
    /// [Viewport
    /// Biasing](https://developers.google.com/maps/documentation/geocoding/intro#Viewports)
    ///
    /// In a Geocoding request, you can instruct the Geocoding service to prefer
    /// results within a given viewport (expressed as a bounding box). You do so
    /// within the request URL by setting the `bounds` parameter. Note that
    /// biasing only _prefers_ results within the bounds; if more relevant
    /// results exist outside of these bounds, they may be included.
    ///
    /// The bounds parameter defines the latitude/longitude coordinates of the
    /// southwest and northeast corners of this bounding box.
    ///
    /// For example, a geocode for "Winnetka" generally returns this suburb of
    /// Chicago. However, adding a `bounds` argument defining a bounding box for
    /// the San Fernando Valley of Los Angeles results in this geocode returning
    /// the neighborhood named "Winnetka" in that location.
    ///
    /// # Example:
    ///
    /// * Specify bounding box for search area:
    /// ```
    /// .with_bounds(Bounds {
    ///     southwest: LatLng { lat: 51.5031117, lng: -0.1291503 },
    ///     northeast: LatLng { lat: 51.5034405, lng: -0.1260032 },
    /// })
    /// ```

    pub fn with_bounds(&mut self, bounds: Bounds) -> &mut ForwardRequest {
        // Set bounds in ForwardRequest struct.
        self.bounds = Some(bounds);
        // Return modified ForwardRequest struct to caller.
        self
    } // fn

} // impl