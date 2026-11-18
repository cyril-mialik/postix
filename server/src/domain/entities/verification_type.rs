use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerificationType {
    Premium,
    Business,
    Famous,
}

impl VerificationType {
    const PREMIUM_IDX: i32 = 1;
    const BUSINESS_IDX: i32 = 2;
    const FAMOUS_IDX: i32 = 3;

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            Self::PREMIUM_IDX => Some(VerificationType::Premium),
            Self::BUSINESS_IDX => Some(VerificationType::Business),
            Self::FAMOUS_IDX => Some(VerificationType::Famous),
            _ => None,
        }
    }

    pub fn as_id(&self) -> i32 {
        match self {
            VerificationType::Premium => Self::PREMIUM_IDX,
            VerificationType::Business => Self::BUSINESS_IDX,
            VerificationType::Famous => Self::FAMOUS_IDX,
        }
    }

    pub fn is_premium(&self) -> bool {
        matches!(self, VerificationType::Premium)
    }

    pub fn is_business(&self) -> bool {
        matches!(self, VerificationType::Business)
    }

    pub fn is_famous(&self) -> bool {
        matches!(self, VerificationType::Famous)
    }
}
