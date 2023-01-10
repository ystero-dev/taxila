/*
 * Common Data Types
 *
 * Common Data Types for Service Based Interfaces. © 2022, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI, TSDSI, TTA, TTC). All rights reserved. 
 *
 * The version of the OpenAPI document: 1.4.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SensorMeasurement : The enumeration SensorMeasurement defines sensor measurement type for MDT in the trace. See 3GPP TS 32.422 for further description of the values. It shall comply with the provisions defined in table 5.6.3.7-1. 

/// The enumeration SensorMeasurement defines sensor measurement type for MDT in the trace. See 3GPP TS 32.422 for further description of the values. It shall comply with the provisions defined in table 5.6.3.7-1. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SensorMeasurement {
    #[serde(rename = "BAROMETRIC_PRESSURE")]
    BarometricPressure,
    #[serde(rename = "UE_SPEED")]
    UeSpeed,
    #[serde(rename = "UE_ORIENTATION")]
    UeOrientation,

}

impl ToString for SensorMeasurement {
    fn to_string(&self) -> String {
        match self {
            Self::BarometricPressure => String::from("BAROMETRIC_PRESSURE"),
            Self::UeSpeed => String::from("UE_SPEED"),
            Self::UeOrientation => String::from("UE_ORIENTATION"),
        }
    }
}

impl Default for SensorMeasurement {
    fn default() -> SensorMeasurement {
        Self::BarometricPressure
    }
}



