/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ComplexQuery : The ComplexQuery data type is either a conjunctive normal form or a disjunctive normal form. The attribute names \"cnfUnits\" and \"dnfUnits\" (see clause 5.2.4.11 and clause 5.2.4.12) serve as discriminator. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ComplexQuery {
    #[serde(rename = "cnfUnits")]
    pub cnf_units: Vec<crate::models::CnfUnit>,
    #[serde(rename = "dnfUnits")]
    pub dnf_units: Vec<crate::models::DnfUnit>,
}

impl ComplexQuery {
    /// The ComplexQuery data type is either a conjunctive normal form or a disjunctive normal form. The attribute names \"cnfUnits\" and \"dnfUnits\" (see clause 5.2.4.11 and clause 5.2.4.12) serve as discriminator. 
    pub fn new(cnf_units: Vec<crate::models::CnfUnit>, dnf_units: Vec<crate::models::DnfUnit>) -> ComplexQuery {
        ComplexQuery {
            cnf_units,
            dnf_units,
        }
    }
}

