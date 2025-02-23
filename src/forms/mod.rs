use crate::prelude::*;
use validator::Validate;

/// The `Login` form is used along with the [`Auth`] guard to authenticate users.
#[derive(FromForm, Deserialize, Clone, Hash, PartialEq, Eq, Validate)]
pub struct Login {
    #[validate(email)]
    pub email: String,
    pub(crate) password: String,
}

/// The `Signup` form is used along with the [`Auth`] guard to create new users.
#[derive(FromForm, Deserialize, Clone, PartialEq, Eq, Hash, Validate)]
pub struct Signup {
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "is_password_secure"))]
    pub(crate) password: String,
}
impl Debug for Signup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Signup {{ email: {:?}, password: \"*****\" }}",
            self.email
        )
    }
}
impl Debug for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Signup {{ email: {:?}, password: \"*****\" }}",
            self.email
        )
    }
}

impl From<Signup> for Login {
    fn from(form: Signup) -> Login {
        Login {
            email: form.email,
            password: form.password,
        }
    }
}

impl From<Login> for Signup {
    fn from(form: Login) -> Signup {
        Self {
            email: form.email,
            password: form.password,
        }
    }
}

impl<T: Deref<Target = Signup>> From<T> for Login {
    fn from(form: T) -> Login {
        Login {
            email: form.email.clone(),
            password: form.password.clone(),
        }
    }
}
pub fn is_password_secure(password: &str) -> Result<(), ValidationError> {
    is_long(password)?;
    has_uppercase(password)?;
    has_lowercase(password)?;
    has_number(password)?;
    Ok(())
}

fn is_long(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        Err(ValidationError::new(
            "The password must be at least 8 characters long.\n",
        ))
    } else {
        Ok(())
    }
}

fn has_uppercase(password: &str) -> Result<(), ValidationError> {
    for c in password.chars() {
        if c.is_uppercase() {
            return Ok(());
        }
    }
    Err(ValidationError::new(
        "The password must include least one uppercase character.\n",
    ))
}

fn has_lowercase(password: &str) -> Result<(), ValidationError> {
    for c in password.chars() {
        if c.is_lowercase() {
            return Ok(());
        }
    }
    // throw!(Error::UnsafePasswordHasNoLower)
    Err(ValidationError::new(
        "The password must include least one uppercase character.\n",
    ))
}

fn has_number(password: &str) -> Result<(), ValidationError> {
    for c in password.chars() {
        if c.is_numeric() {
            return Ok(());
        }
    }
    Err(ValidationError::new(
        "The password has to contain at least one digit.\n",
    ))
    // throw!(Error::UnsafePasswordHasNoDigit)
}
