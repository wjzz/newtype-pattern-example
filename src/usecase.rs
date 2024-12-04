use crate::domain::{Email, FirstName, LastName, RegisteredUser};
use crate::infra::RegisterMailMagazineCommand;
use crate::util::NonEmptyString;

pub enum RegistrationError {
    FirstNameError(String),
    LastNameError(String),
    EmailError(String),
}

pub fn try_register_use(
    register_cmd: RegisterMailMagazineCommand,
) -> Result<RegisteredUser, RegistrationError> {
    let first_name = FirstName(
        NonEmptyString::try_from_string(register_cmd.first_name)
            .map_err(RegistrationError::FirstNameError)?,
    );

    let last_name = LastName(
        NonEmptyString::try_from_string(register_cmd.last_name)
            .map_err(RegistrationError::LastNameError)?,
    );

    let email =
        Email::try_from_string(register_cmd.email).map_err(RegistrationError::EmailError)?;

    Ok({
        RegisteredUser {
            first_name,
            last_name,
            email,
        }
    })
}
