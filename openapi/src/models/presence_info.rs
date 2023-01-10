/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PresenceInfo : If the additionalPraId IE is present, this IE shall state the presence information of the UE for the individual PRA identified by the additionalPraId IE;  If the additionalPraId IE is not present, this IE shall state the presence information of the UE for the PRA identified by the praId IE. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PresenceInfo {
    /// Represents an identifier of the Presence Reporting Area (see clause 28.10 of 3GPP TS 23.003.  This IE shall be present  if the Area of Interest subscribed or reported is a Presence Reporting Area or a Set of Core Network predefined Presence Reporting Areas. When present, it shall be encoded as a string representing an integer in the following ranges: 0 to 8 388 607 for UE-dedicated PRA 8 388 608 to 16 777 215 for Core Network predefined PRA Examples: PRA ID 123 is encoded as \"123\" PRA ID 11 238 660 is encoded as \"11238660\" 
    #[serde(rename = "praId", skip_serializing_if = "Option::is_none")]
    pub pra_id: Option<String>,
    /// This IE may be present if the praId IE is present and if it contains a PRA identifier referring to a set of Core Network predefined Presence Reporting Areas. When present, this IE shall contain a PRA Identifier of an individual PRA within the Set of Core Network predefined Presence Reporting Areas indicated by the praId IE. 
    #[serde(rename = "additionalPraId", skip_serializing_if = "Option::is_none")]
    pub additional_pra_id: Option<String>,
    #[serde(rename = "presenceState", skip_serializing_if = "Option::is_none")]
    pub presence_state: Option<crate::models::PresenceState>,
    /// Represents the list of tracking areas that constitutes the area. This IE shall be present if the subscription or  the event report is for tracking UE presence in the tracking areas. For non 3GPP access the TAI shall be the N3GPP TAI. 
    #[serde(rename = "trackingAreaList", skip_serializing_if = "Option::is_none")]
    pub tracking_area_list: Option<Vec<crate::models::Tai>>,
    /// Represents the list of EUTRAN cell Ids that constitutes the area. This IE shall be present if the Area of Interest subscribed is a list of EUTRAN cell Ids. 
    #[serde(rename = "ecgiList", skip_serializing_if = "Option::is_none")]
    pub ecgi_list: Option<Vec<crate::models::Ecgi>>,
    /// Represents the list of NR cell Ids that constitutes the area. This IE shall be present if the Area of Interest subscribed is a list of NR cell Ids. 
    #[serde(rename = "ncgiList", skip_serializing_if = "Option::is_none")]
    pub ncgi_list: Option<Vec<crate::models::Ncgi>>,
    /// Represents the list of NG RAN node identifiers that constitutes the area. This IE shall be present if the Area of Interest subscribed is a list of NG RAN node identifiers. 
    #[serde(rename = "globalRanNodeIdList", skip_serializing_if = "Option::is_none")]
    pub global_ran_node_id_list: Option<Vec<crate::models::GlobalRanNodeId>>,
    /// Represents the list of eNodeB identifiers that constitutes the area. This IE shall be present if the Area of Interest subscribed is a list of eNodeB identifiers. 
    #[serde(rename = "globaleNbIdList", skip_serializing_if = "Option::is_none")]
    pub globale_nb_id_list: Option<Vec<crate::models::GlobalRanNodeId>>,
}

impl PresenceInfo {
    /// If the additionalPraId IE is present, this IE shall state the presence information of the UE for the individual PRA identified by the additionalPraId IE;  If the additionalPraId IE is not present, this IE shall state the presence information of the UE for the PRA identified by the praId IE. 
    pub fn new() -> PresenceInfo {
        PresenceInfo {
            pra_id: None,
            additional_pra_id: None,
            presence_state: None,
            tracking_area_list: None,
            ecgi_list: None,
            ncgi_list: None,
            global_ran_node_id_list: None,
            globale_nb_id_list: None,
        }
    }
}

