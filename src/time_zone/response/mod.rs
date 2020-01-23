//! Resources (enums, structs) for the client process the Time Zone API response
//! from the Google Cloud server.

pub mod status;

use crate::time_zone::response::status::Status;
use serde::{Serialize, Deserialize};

/// The response from the Google Maps Time Zone API will be stored in this
/// structure.
///
/// [Time Zone Responses](https://developers.google.com/maps/documentation/timezone/intro#Responses)
/// ------------------------------------------------------------------------------------------------
/// For each valid request, the time zone service will return a response in the
/// format indicated within the request URI.
///
/// [Calculating the Local Time](https://developers.google.com/maps/documentation/timezone/intro#CalculatingTime)
///
/// The local time of a given location is the sum of the timestamp parameter,
/// and the dstOffset and rawOffset fields from the result.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {

    /// The offset for daylight-savings time in seconds. This will be zero if
    /// the time zone is not in Daylight Savings Time during the specified
    /// `time`.
    #[serde(alias = "dstOffset")]
    pub dst_offset: Option<i16>,

    /// More detailed information about the reasons behind the given status
    /// code, if other than `OK`.
    #[serde(alias = "errorMessage")]
    pub error_message: Option<String>,

    /// The offset from UTC (in seconds) for the given location. This does not
    /// take into effect daylight savings.
    #[serde(alias = "rawOffset")]
    pub raw_offset: Option<i16>,

    /// The status of the response.
    pub status: Status,

    /// A string containing the ID of the time zone, such as
    /// "America/Los_Angeles" or "Australia/Sydney". These IDs are defined by
    /// Unicode [Common Locale Data Repository (CLDR)
    /// project](http://cldr.unicode.org/), and currently available in file
    /// [timezone.xml](http://unicode.org/repos/cldr/trunk/common/bcp47/timezone.xml).
    /// When a timezone has several IDs, the canonical one is returned. In
    /// timezone.xml, this is the first alias of each timezone. For example,
    /// "Asia/Calcutta" is returned, not "Asia/Kolkata".
    #[serde(alias = "timeZoneId")]
    pub time_zone_id: Option<String>,

    /// A string containing the long form name of the time zone. This field will
    /// be localized if the language parameter is set. eg. "Pacific Daylight
    /// Time" or "Australian Eastern Daylight Time"
    #[serde(alias = "timeZoneName")]
    pub time_zone_name: Option<String>,

} // struct