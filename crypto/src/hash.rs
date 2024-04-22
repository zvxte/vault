use argon2::{
    password_hash::{
        rand_core::OsRng,
        Error, PasswordHash, PasswordHasher, SaltString
    },
    Argon2, PasswordVerifier,
};

pub trait Hasher {
    fn hash_password(&self, password: &String) -> Result<String, Error>;
    fn cmp_password(&self, plain_password: &String, hashed_password: &String) -> Result<bool, Error>;
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
    fn hash_password(&self, password: &String) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password =
            self.argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(hashed_password.to_string())
    }

    fn cmp_password(&self, plain_password: &String, hashed_password: &String) -> Result<bool, Error> {
        let parsed_hashed_password = PasswordHash::new(&hashed_password)?;
        Ok(self.argon2.verify_password(
            plain_password.as_bytes(),
            &parsed_hashed_password,
        ).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argon2_hasher() {
        let argon2 = Argon2Hasher::new();

        let plain_password = "mve53!#*qwp627.[fgm31".to_string();
        let hashed_password = argon2.hash_password(&plain_password).unwrap();
        let result = argon2.cmp_password(&plain_password, &hashed_password).unwrap();
        assert_eq!(result, true);

        let plain_password = "mve53!#*qwp627.[fgm31".to_string();
        let hashed_password = argon2.hash_password(&plain_password).unwrap();
        let result = argon2.cmp_password(&"lin354v2v23c@^Y".to_string(), &hashed_password).unwrap();
        assert_eq!(result, false);
    }
}
