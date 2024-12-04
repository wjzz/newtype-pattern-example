// This object is coming from outside, we cannot control the contents,
// so we use primitive types here

pub struct RegisterMailMagazineCommand {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
