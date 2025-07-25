use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::helpers::regex::{is_email_valid, is_name_valid, is_password_valid};

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Validate for UserInput {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if !is_name_valid(&self.name) {
            errors.add("name", ValidationError::new("Invalid name"));
        }

        if !is_email_valid(&self.email) {
            errors.add("email", ValidationError::new("Invalid email"));
        }

        if !is_password_valid(&self.password) {
            errors.add("password", ValidationError::new("Invalid password"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
