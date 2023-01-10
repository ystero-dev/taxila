/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RefToBinaryDataRm : This data type is defined in the same way as the ' RefToBinaryData ' data type, but with the OpenAPI 'nullable: true' property. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RefToBinaryDataRm {
    /// This IE shall contain the value of the Content-ID header of the referenced binary body part. 
    #[serde(rename = "contentId")]
    pub content_id: String,
}

impl RefToBinaryDataRm {
    /// This data type is defined in the same way as the ' RefToBinaryData ' data type, but with the OpenAPI 'nullable: true' property. 
    pub fn new(content_id: String) -> RefToBinaryDataRm {
        RefToBinaryDataRm {
            content_id,
        }
    }
}

