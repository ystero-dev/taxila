/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Point : Ellipsoid Point.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    #[serde(rename = "shape")]
    pub shape: Box<crate::models::SupportedGadShapes>,
    #[serde(rename = "point")]
    pub point: Box<crate::models::GeographicalCoordinates>,
}

impl Point {
    /// Ellipsoid Point.
    pub fn new(shape: crate::models::SupportedGadShapes, point: crate::models::GeographicalCoordinates) -> Point {
        Point {
            shape: Box::new(shape),
            point: Box::new(point),
        }
    }
}

