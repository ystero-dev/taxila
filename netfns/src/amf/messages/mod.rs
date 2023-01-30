//! Message Definitions for Messages sent by Individual Tasks

use sctp_rs::{AssociationId, ReceivedData, SendData};

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
pub(crate) enum AmfToNgapMessage {
    Signal(i32),
}

// Message sent to Ran Connection task by NGAP Task.
#[derive(Debug, Clone)]
pub(crate) enum NgapMgrToRanConnMessage {
    SendData(SendDataMessage),
    Signal(i32),
}

#[derive(Debug, Clone)]
pub(crate) struct SendDataMessage {
    pub(crate) _id: AssociationId,
    pub(crate) txdata: SendData,
}

// Message Sent from AMF to NAS Task
#[derive(Debug)]
pub(crate) enum AmfToNasMessage {
    Signal(i32),
}

// Message Sent from NAS to AMF
#[derive(Debug)]
pub(crate) enum NasToAmfMessage {}
