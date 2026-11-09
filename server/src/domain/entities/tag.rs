use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Tag {
    pub id: i32,

    #[validate(length(min = 1, max = 50))]
    pub name: String,
}

impl Tag {
    pub fn new(name: &str) -> Result<Self, TagError> {
        let name = name.trim().to_lowercase();
        let tag = Self { id: 0, name };

        tag.validate().map_err(|e| {
            let field_errors = e.field_errors();

            if field_errors.contains_key("name") {
                return TagError::InvalidNameLength;
            }

            TagError::Validation(e.to_string())
        })?;

        Ok(tag)
    }
}

#[derive(Debug, Error)]
pub enum TagError {
    #[error("Неверная длина тега (1–50)")]
    InvalidNameLength,

    #[error("Тег уже существует")]
    AlreadyExists,

    #[error("Тег не найден")]
    NotFound,

    #[error("Ошибка валидации: {0}")]
    Validation(String),
}
