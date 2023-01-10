/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RoamingRestrictions : Indicates if access is allowed to a given serving network, e.g. a PLMN (MCC, MNC) or an SNPN (MCC, MNC, NID). 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoamingRestrictions {
    #[serde(rename = "accessAllowed", skip_serializing_if = "Option::is_none")]
    pub access_allowed: Option<bool>,
}

impl RoamingRestrictions {
    /// Indicates if access is allowed to a given serving network, e.g. a PLMN (MCC, MNC) or an SNPN (MCC, MNC, NID). 
    pub fn new() -> RoamingRestrictions {
        RoamingRestrictions {
            access_allowed: None,
        }
    }
}

