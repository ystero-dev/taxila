/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NetworkId : contains PLMN and Network identity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkId {
    /// Mobile Network Code part of the PLMN, comprising 2 or 3 digits, as defined in clause 9.3.3.5 of 3GPP TS 38.413.
    #[serde(rename = "mnc", skip_serializing_if = "Option::is_none")]
    pub mnc: Option<String>,
    /// Mobile Country Code part of the PLMN, comprising 3 digits, as defined in clause 9.3.3.5 of 3GPP TS 38.413. 
    #[serde(rename = "mcc", skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
}

impl NetworkId {
    /// contains PLMN and Network identity.
    pub fn new() -> NetworkId {
        NetworkId {
            mnc: None,
            mcc: None,
        }
    }
}

