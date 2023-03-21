//! Handling of Registration Procedure as defined in 24.501
//!
//! Section 5.5.1.2 For Initial Registration
//! Section 5.5.1.3 Mobility Registration and Periodic Registration Update
//!
//! Also: Section 4.2.2.2 from 23.502 Release 17. (Note: General Registration Only).
//!
use super::AmfUe;
use nas::messages::RegistrationRequest;

impl AmfUe {
    pub(super) fn registration_procedure(
        &mut self,
        reg_request: RegistrationRequest,
        initial: bool,
    ) -> std::io::Result<()> {
        if initial {
            self.initial_registration_procedure(reg_request)
        } else {
            self.periodic_or_mobility_registration_procedure(reg_request)
        }
    }

    fn initial_registration_procedure(
        &mut self,
        reg_request: RegistrationRequest,
    ) -> std::io::Result<()> {
        log::debug!("Registration Request: {:#?}", reg_request);
        Ok(())
    }

    fn periodic_or_mobility_registration_procedure(
        &mut self,
        reg_request: RegistrationRequest,
    ) -> std::io::Result<()> {
        todo!()
    }
}
