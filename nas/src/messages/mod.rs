mod ies;

pub mod headers;

pub mod registration_request;
pub use registration_request::RegistrationRequest;

pub const MM_MSG_TYPE_REGISTRATION_REQUEST: u8 = 0x41;
