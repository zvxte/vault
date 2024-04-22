use aes_gcm::{
    aead::Aead,
    AeadCore, Aes256Gcm, Key, KeyInit,
};
use argon2::{
    password_hash::rand_core::OsRng,
    Argon2,
};

pub trait Encrypter {
    fn encrypt(&self, plain_password: String) -> Result<EncryptedData, aes_gcm::Error>;
    fn decrypt(&self, encrypted_data: EncryptedData) -> Result<String, aes_gcm::Error>;
}

pub struct AesGcmEncrypter {
    key: Key<Aes256Gcm>,
}

impl AesGcmEncrypter {
    fn _build(plain_password: String, salt: String) -> Result<Self, argon2::Error> {
        let mut key = [0u8; 32];
        Argon2::default().hash_password_into(
            plain_password.as_bytes(), 
            salt.as_bytes(), 
            &mut key,
        )?;
        let key = key.into();
        Ok(Self { key })
    }
}

impl Encrypter for AesGcmEncrypter {
    fn encrypt(&self, content: String) -> Result<EncryptedData, aes_gcm::Error> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let data = cipher.encrypt(
            &nonce,
            content.as_bytes().as_ref(),
        )?;
        Ok(EncryptedData::new(nonce.into(), data))
    }
    
    fn decrypt(&self, encrypted_data: EncryptedData)
    -> Result<String, aes_gcm::Error> {
        let cipher = Aes256Gcm::new(&self.key);
        let data = cipher.decrypt(
            &encrypted_data.nonce.into(),
            encrypted_data.content.as_ref()
        )?;
        Ok(String::from_utf8(data).unwrap())
    }
}

pub struct EncryptedData {
    nonce: [u8; 12],
    content: Vec<u8>,
}

impl EncryptedData {
    fn new(nonce: [u8; 12], content: Vec<u8>) -> Self {
        Self { nonce, content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption_decryption() {
        let encrypter = AesGcmEncrypter::_build(
            "my_master_password".to_string(),
            "my_master_salt".to_string(),
        ).unwrap();
        let plain_password = "my_password".to_string();
        let encrypted_data = encrypter.encrypt(plain_password.clone()).unwrap();
        let decrypted_password = encrypter.decrypt(encrypted_data).unwrap();
        println!("{}", decrypted_password);

        assert_eq!(plain_password, decrypted_password);
    }
}
