#[derive(Debug)]
pub enum UserError {
    InvalidEmail,
    PasswordTooShort,
    PasswordTooLong,
}