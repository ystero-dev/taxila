//! Message Definitions for Messages sent by Individual Tasks

use ngap::messages::r17::NGAP_PDU;

use sctp_rs::{AssociationId, ReceivedData};

// Message sent by NGAP Task to AMF
#[derive(Debug)]
pub(crate) enum NgapToAmfMessage {
    PDU(PDUMessage),
}

#[derive(Debug)]
pub(crate) struct PDUMessage {
    pub(crate) id: AssociationId,
    pub(crate) pdu: NGAP_PDU,
}

// Message sent to NGAP by GNB Connection Task
#[derive(Debug, Clone)]
pub(crate) enum GnbToNgapMessage {
    ReceivedData(ReceivedDataMessage),
}

#[derive(Debug, Clone)]
pub(crate) struct ReceivedDataMessage {
    pub(crate) id: AssociationId,
    pub(crate) rxdata: ReceivedData,
}

// Message sent to NGAP Task by AMF.
#[derive(Debug, Clone)]
pub(crate) enum AmfToNgapMessage {}

// Message sent to Gnb Connection task by NGAP Task.
#[derive(Debug, Clone)]
pub(crate) enum NgapToGnbMessage {}
