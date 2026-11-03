use anyhow::{Result, anyhow};
use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

pub fn hash_password(plain: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|e| anyhow!("Failed to hash password: {}", e))?
        .to_string();

    Ok(hash)
}

pub fn verify_password(plain: &str, hash: &str) -> Result<bool> {
    let parsed = PasswordHash::new(hash).map_err(|e| anyhow!("Failed to parse hash: {}", e))?;

    Ok(Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("password", "password", true)]
    #[case("password", "invalid", false)]
    fn test_hash_password(
        #[case] password: &str,
        #[case] expected_password: &str,
        #[case] expected: bool,
    ) {
        let hash = hash_password(password).unwrap();

        assert!(verify_password(expected_password, &hash).unwrap() == expected);
    }

    #[rstest]
    #[case("password", "password", true)]
    #[case("password", "invalid", false)]
    #[rstest]
    fn test_verify_password(
        #[case] password: &str,
        #[case] expected_password: &str,
        #[case] expected: bool,
    ) {
        let hash = hash_password(password).unwrap();

        assert!(verify_password(expected_password, &hash).unwrap() == expected);
    }
}
