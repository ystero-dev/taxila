/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PartitioningCriteria : Possible values are: - \"TAC\": Type Allocation Code - \"SUBPLMN\": Subscriber PLMN ID - \"GEOAREA\": Geographical area, i.e. list(s) of TAI(s) - \"SNSSAI\": S-NSSAI - \"DNN\": DNN 

/// Possible values are: - \"TAC\": Type Allocation Code - \"SUBPLMN\": Subscriber PLMN ID - \"GEOAREA\": Geographical area, i.e. list(s) of TAI(s) - \"SNSSAI\": S-NSSAI - \"DNN\": DNN 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PartitioningCriteria {
    #[serde(rename = "TAC")]
    Tac,
    #[serde(rename = "SUBPLMN")]
    Subplmn,
    #[serde(rename = "GEOAREA")]
    Geoarea,
    #[serde(rename = "SNSSAI")]
    Snssai,
    #[serde(rename = "DNN")]
    Dnn,

}

impl ToString for PartitioningCriteria {
    fn to_string(&self) -> String {
        match self {
            Self::Tac => String::from("TAC"),
            Self::Subplmn => String::from("SUBPLMN"),
            Self::Geoarea => String::from("GEOAREA"),
            Self::Snssai => String::from("SNSSAI"),
            Self::Dnn => String::from("DNN"),
        }
    }
}

impl Default for PartitioningCriteria {
    fn default() -> PartitioningCriteria {
        Self::Tac
    }
}



