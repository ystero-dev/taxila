/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TnapId : Contain the TNAP Identifier see clause5.6.2 of 3GPP TS 23.501.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TnapId {
    /// This IE shall be present if the UE is accessing the 5GC via a trusted WLAN access network.When present, it shall contain the SSID of the access point to which the UE is attached, that is received over NGAP,  see IEEE Std 802.11-2012. 
    #[serde(rename = "ssId", skip_serializing_if = "Option::is_none")]
    pub ss_id: Option<String>,
    /// When present, it shall contain the BSSID of the access point to which the UE is attached, that is received over NGAP, see IEEE Std 802.11-2012. 
    #[serde(rename = "bssId", skip_serializing_if = "Option::is_none")]
    pub bss_id: Option<String>,
    /// string with format 'bytes' as defined in OpenAPI
    #[serde(rename = "civicAddress", skip_serializing_if = "Option::is_none")]
    pub civic_address: Option<String>,
}

impl TnapId {
    /// Contain the TNAP Identifier see clause5.6.2 of 3GPP TS 23.501.
    pub fn new() -> TnapId {
        TnapId {
            ss_id: None,
            bss_id: None,
            civic_address: None,
        }
    }
}

