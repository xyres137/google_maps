//! Contains the `Bounds` struct and its associated traits. It is used to
//! specify a selection or bounding box over a geographic area using two
//! latitude & longitude pairs.

#[cfg(feature = "geo")]
mod geo_conversions;

// -----------------------------------------------------------------------------

use crate::types::error::Error;
use crate::types::latlng::LatLng;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------

/// Contains the recommended viewport for displaying the returned result,
/// specified as two latitude & longitude pairs defining the southwest and
/// northeast corner of the viewport bounding box. Generally the viewport is
/// used to frame a result when displaying it to a user.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Bounds {
    /// South-west or bottom-left corner of the bounding box.
    pub southwest: LatLng,
    /// North-east or top-right corner of the bounding box.
    pub northeast: LatLng,
} // struct

// -----------------------------------------------------------------------------

impl std::fmt::Display for Bounds {
    /// Converts a `Bounds` struct to a `String` that contains two
    /// latitude & longitude pairs that represent a bounding box.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "{},{}|{},{}",
            self.southwest.lat,
            self.southwest.lng,
            self.northeast.lat,
            self.northeast.lng,
        ) // write!
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Bounds> for String {
    /// Converts a `Bounds` struct to a `String` that contains two
    /// latitude & longitude pairs that represent a bounding box.
    fn from(bounds: &Bounds) -> Self {
        bounds.to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for Bounds {
    // Error definitions are contained in the `type_error.rs` module.
    type Err = crate::types::Error;
    /// Gets a `Bounds` struct from a `String` that contains two pipe-delimited
    /// latitude & longitude pairs.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let corner: Vec<&str> = value.trim()
            .split('|')
            .collect();
        if corner.len() != 2 {
            Err(Error::InvalidBoundsString(value.to_owned()))
        } else {
            let southwest = LatLng::from_str(corner[0].trim());
            let southwest = southwest.map_err(|_| Error::InvalidBoundsString(value.to_owned()))?;
            let northeast = LatLng::from_str(corner[1].trim());
            let northeast = northeast.map_err(|_| Error::InvalidBoundsString(value.to_owned()))?;
            Ok(Bounds { southwest, northeast })
        } // if
    } // fn
} // impl