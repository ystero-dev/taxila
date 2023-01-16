use std::net::IpAddr;

use serde::{
    de::{Deserializer, Error},
    Deserialize, Serialize,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct AmfIdConfig {
    pub(crate) pointer: u8,
    pub(crate) set: u16,
    pub(crate) region: u8,
}

impl<'de> Deserialize<'de> for AmfIdConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        // Pointer can at the most be 6 bits.
        if this.pointer > 63 {
            return Err(D::Error::custom("Max supported value for `pointer` is 63."));
        }

        // Set can be at the most 10 bits.
        if this.set > 1024 {
            return Err(D::Error::custom("Max supported value for `pointer` is 63."));
        }

        Ok(this)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct PlmnConfig {
    pub(crate) mcc: u16,
    pub(crate) mnc: u16,
}

impl<'de> Deserialize<'de> for PlmnConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        if this.mcc > 999 || this.mnc > 999 {
            return Err(D::Error::custom(
                "Max supported value for `mcc` and `mnc` is 999.",
            ));
        }

        Ok(this)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct NgapConfig {
    pub(crate) addrs: Vec<IpAddr>,
    pub(crate) port: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(remote = "Self")]
pub struct AmfConfig {
    pub(crate) ngap: NgapConfig,
    pub(crate) plmn: PlmnConfig,
    pub(crate) tacs: Vec<u32>, // TODO: Validate Max value is 24 bit.
    pub(crate) amf_id: AmfIdConfig,
    pub(crate) amf_name: String,
}

impl<'de> Deserialize<'de> for AmfConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        let max_tac_value = (1u32 << 24) - 1;
        for tac in &this.tacs {
            if tac > &max_tac_value {
                return Err(D::Error::custom(format!(
                    "Max supported value for `tac` should be less than {}",
                    max_tac_value + 1
                )));
            }
        }

        Ok(this)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn works() {
        let config_str =
            "ngap:\n addrs:\n - 127.0.0.1 \n - ::1 \nport: 38413\nplmn:\n mcc: 999\n mnc: 99\ntacs: [ 1, 2, 3]\namf_id:\n pointer: 63\n set: 10\n region: 1\namf_name: taxila-amf";
        let amf_config: Result<super::AmfConfig, _> = serde_yaml::from_str(config_str);
        assert!(amf_config.is_ok(), "{:#?}", amf_config.err().unwrap());
    }
}
