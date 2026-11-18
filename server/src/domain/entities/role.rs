use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Moderator,
    Admin,
}

impl Role {
    const USER: &str = "user";
    const MODERATOR: &str = "moderator";
    const ADMIN: &str = "admin";

    const USER_IDX: i32 = 1;
    const MODERATOR_IDX: i32 = 2;
    const ADMIN_IDX: i32 = 3;

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            Self::USER_IDX => Some(Role::User),
            Self::MODERATOR_IDX => Some(Role::Moderator),
            Self::ADMIN_IDX => Some(Role::Admin),
            _ => None,
        }
    }

    pub fn as_id(&self) -> i32 {
        match self {
            Role::User => Self::USER_IDX,
            Role::Moderator => Self::MODERATOR_IDX,
            Role::Admin => Self::ADMIN_IDX,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Role::User => Self::USER,
            Role::Moderator => Self::MODERATOR,
            Role::Admin => Self::ADMIN,
        }
    }

    pub fn is_moderator_or_admin(&self) -> bool {
        self.is_admin() || self.is_moderator()
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }

    pub fn is_user(&self) -> bool {
        matches!(self, Role::User)
    }

    pub fn is_moderator(&self) -> bool {
        matches!(self, Role::Moderator)
    }
}
