/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AcsInfo : The ACS information for the 5G-RG is defined in BBF TR-069 [42] or in BBF TR-369



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AcsInfo {
    /// String providing an URI formatted according to RFC 3986.
    #[serde(rename = "acsUrl", skip_serializing_if = "Option::is_none")]
    pub acs_url: Option<String>,
    /// String identifying a IPv4 address formatted in the 'dotted decimal' notation as defined in RFC 1166. 
    #[serde(rename = "acsIpv4Addr", skip_serializing_if = "Option::is_none")]
    pub acs_ipv4_addr: Option<String>,
    #[serde(rename = "acsIpv6Addr", skip_serializing_if = "Option::is_none")]
    pub acs_ipv6_addr: Option<Box<crate::models::Ipv6Addr>>,
}

impl AcsInfo {
    /// The ACS information for the 5G-RG is defined in BBF TR-069 [42] or in BBF TR-369
    pub fn new() -> AcsInfo {
        AcsInfo {
            acs_url: None,
            acs_ipv4_addr: None,
            acs_ipv6_addr: None,
        }
    }
}

