use bitvec::prelude::{BitVec, Msb0};

use crate::messages::r17::{AMFPointer, AMFRegionID, AMFSetID, PLMNIdentity, GUAMI};

impl GUAMI {
    /// GUAMI From the PLMN ID (MCC, MNC) and AMF Region, AMF Set and AMF Pointer)
    pub fn from_plmn_id_and_amf_id(mcc: u16, mnc: u16, region: u8, set: u16, pointer: u8) -> Self {
        let region_bv = BitVec::<_, Msb0>::from_element(region);
        let mut set_bv = BitVec::<_, Msb0>::from_vec(set.to_be_bytes().to_vec());
        set_bv.retain(|idx, _| idx >= 6);
        let mut pointer_bv = BitVec::<_, Msb0>::from_element(pointer);
        pointer_bv.retain(|idx, _| idx >= 2);

        Self {
            plmn_identity: PLMNIdentity::from_mcc_mnc(mcc, mnc),
            amf_region_id: AMFRegionID(region_bv),
            amf_set_id: AMFSetID(set_bv),
            amf_pointer: AMFPointer(pointer_bv),
            ie_extensions: None,
        }
    }
}
