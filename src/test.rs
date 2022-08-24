#[cfg(test)]
mod test {

    use crate::crypto::{decrypt, encrypt};
    use uuid::Uuid;

    #[test]
    fn test() {
        let id = Uuid::new_v4();
        let key32 = id.simple().to_string();
        let key = key32.split_at(16).0;
        println!("UUID:{:#?}", key);
        let encrypt = encrypt(key.as_bytes(), "Hello,World".as_bytes()).unwrap_or_default();
        let result = hex::encode(encrypt.clone());
        println!("Encrypted:{:?},Encrypted(Hex):{:#?}",encrypt, result);
        let decrypt = decrypt(key.as_bytes(),&*hex::decode(result).unwrap_or_default()).unwrap_or_default();
        println!("Decrypted:{:?},String:{}",decrypt,String::from_utf8(decrypt.clone()).unwrap_or_default()
        )
    }
}
