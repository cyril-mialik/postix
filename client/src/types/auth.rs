use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}
