#![allow(dead_code, unreachable_patterns, non_camel_case_types)]

use std::convert::TryInto;

pub mod r17 {
    // TODO: Include the output of `build.rs` here.
    include!(concat!(env!("OUT_DIR"), "/ngap.rs"));
}

use r17::PLMNIdentity;

impl PLMNIdentity {
    pub fn from_mcc_mnc(mcc: u16, mnc: u16) -> Self {
        let mcc3 = mcc % 10;
        let mcc2 = (mcc % 100) / 10;
        let mcc1 = (mcc % 1000) / 100;

        let mnc3 = mnc % 10;
        let mnc2 = (mnc % 100) / 10;
        let mut mnc1 = (mnc % 1000) / 100;

        if mnc1 == 0 {
            mnc1 = 0xf;
        }

        let byte0 = mcc2 << 4 | mcc1;
        let byte1 = mnc1 << 4 | mcc3;
        let byte2 = mnc3 << 4 | mnc2;

        Self(vec![
            byte0.try_into().unwrap(),
            byte1.try_into().unwrap(),
            byte2.try_into().unwrap(),
        ])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn plmn_identity_from_values() {
        struct PlmnIdentityTestValues {
            mcc: u16,
            mnc: u16,
            id: PLMNIdentity,
        }

        let values = vec![
            PlmnIdentityTestValues {
                mcc: 901,
                mnc: 70,
                id: PLMNIdentity(vec![9, 241, 7]),
            },
            PlmnIdentityTestValues {
                mcc: 286,
                mnc: 01,
                id: PLMNIdentity(vec![130, 246, 16]),
            },
            PlmnIdentityTestValues {
                mcc: 286,
                mnc: 101,
                id: PLMNIdentity(vec![130, 22, 16]),
            },
        ];

        for value in values {
            let plmnid = PLMNIdentity::from_mcc_mnc(value.mcc, value.mnc);
            assert!(
                plmnid.0 == value.id.0,
                "returned: {:?}, expected: {:?}",
                plmnid.0,
                value.id.0
            );
        }
    }
}
