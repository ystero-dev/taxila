/// NAS 5GS Session Management header.
use super::ExtProtoDiscriminator;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Nas5gSmMessageHeader {
    pub extended_protocol_disc: ExtProtoDiscriminator,
    pub pdu_session_identity: u8,
    pub proc_transaction_identity: u8,
}
