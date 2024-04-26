use argon2::{
    password_hash::{
        rand_core::OsRng,
        Error, PasswordHash, PasswordHasher, SaltString
    },
    Argon2, PasswordVerifier,
};
use sha3::{Sha3_256, Digest};

pub trait Hasher {
    fn hash_data(&self, data: &String) -> Result<String, Error>;
    fn cmp_data(&self, plain_data: &String, hashed_data: &String) -> Result<bool, Error>;
}

#[derive(Clone)]
pub struct Argon2Hasher<'a> {
    argon2: Argon2<'a>,
}

impl<'a> Argon2Hasher<'a> {
    pub fn new() -> Self {
        Self { argon2: Argon2::default() }
    }
}

impl<'a> Hasher for Argon2Hasher<'a> {
    fn hash_data(&self, data: &String) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password =
            self.argon2.hash_password(data.as_bytes(), &salt)?;
        Ok(hashed_password.to_string())
    }

    fn cmp_data(&self, plain_data: &String, hashed_data: &String) -> Result<bool, Error> {
        let parsed_hashed_data = PasswordHash::new(&hashed_data)?;
        Ok(self.argon2.verify_password(
            plain_data.as_bytes(),
            &parsed_hashed_data,
        ).is_ok())
    }
}

pub fn hash_with_sha3(data: &String) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argon2_hasher() {
        let argon2 = Argon2Hasher::new();

        let plain_data = "mve53!#*qwp627.[fgm31".to_string();
        let hashed_data = argon2.hash_data(&plain_data).unwrap();
        let result = argon2.cmp_data(&plain_data, &hashed_data).unwrap();
        assert_eq!(result, true);

        let plain_data = "mve53!#*qwp627.[fgm31".to_string();
        let hashed_data = argon2.hash_data(&plain_data).unwrap();
        let result = argon2.cmp_data(&"lin354v2v23c@^Y".to_string(), &hashed_data).unwrap();
        assert_eq!(result, false);
    }
}
