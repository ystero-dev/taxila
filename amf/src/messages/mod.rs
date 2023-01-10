//! Message Definitions for Messages sent by Individual Tasks

use sctp_rs::{AssociationId, ReceivedData};

// Message sent by NGAP Task to AMF
#[derive(Debug)]
pub(crate) enum NgapToAmfMessage {}

// Message sent to NGAP by RAN Connection Task
#[derive(Debug, Clone)]
pub(crate) enum RanConnToNgapMgrMessage {
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
pub(crate) enum NgapMgrToRanConnMessage {}
