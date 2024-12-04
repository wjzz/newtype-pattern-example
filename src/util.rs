// Note: the wrapped value is made private
// This way we cannot circumvent the smart constructor
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn try_from_string(first_name: String) -> Result<NonEmptyString, String> {
        if first_name == "" {
            Err(String::from("NonEmptyString: given empty string"))
        } else {
            Ok(NonEmptyString(first_name))
        }
    }
}
