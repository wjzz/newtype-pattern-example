use crate::util::NonEmptyString;

pub struct FirstName(pub NonEmptyString);
pub struct LastName(pub NonEmptyString);

pub struct Email {
    username: NonEmptyString,
    domain: NonEmptyString,
}

impl Email {
    pub fn try_from_string(email: String) -> Result<Email, String> {
        if email == "" {
            return Err(String::from("Email: empty string given"));
        }
        if !email.contains("@") {
            return Err(format!("Email: not @ mark {}", email));
        }

        let parts: Vec<&str> = email.split("@").collect();
        // skip some validation for simplicity
        if parts.len() < 2 {
            return Err(format!("Not enough parts: {}", email));
        }
        let username = NonEmptyString::try_from_string(String::from(parts[0]))?;
        let domain = NonEmptyString::try_from_string(String::from(parts[1]))?;
        Ok(Email { username, domain })
    }
}

pub struct RegisteredUser {
    pub first_name: FirstName,
    pub last_name: LastName,
    pub email: Email,
}
