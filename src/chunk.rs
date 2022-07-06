use openssl::symm::{decrypt, encrypt, Cipher};
use std::convert::TryFrom;

use crate::{AesKey, Error, AES_KEY_LENGTH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub total_chunks: u64,
    pub index: u64,
    pub key: AesKey,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn to_buf(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(self.total_chunks.to_be_bytes().iter());
        out.extend(self.index.to_be_bytes().iter());
        out.extend(self.key.iter());
        out.extend(self.data.iter());

        out
    }

    pub fn from_buf(buf: &[u8]) -> Result<Self, Error> {
        // Check if the buffer is smaller than u64 (total_chunks)+ u64 (index) + AesKey
        if buf.len() < 8 + 8 + AES_KEY_LENGTH {
            return Err(Error::ChunkParseError);
        }

        let total_chunks = u64::from_be_bytes(
            <[u8; 8]>::try_from(&buf[0..8]).map_err(|_| Error::ChunkParseError)?,
        );
        let index = u64::from_be_bytes(
            <[u8; 8]>::try_from(&buf[8..16]).map_err(|_| Error::ChunkParseError)?,
        );
        let key: AesKey = <AesKey>::try_from(&buf[16..16 + AES_KEY_LENGTH])
            .map_err(|_| Error::ChunkParseError)?;
        let data = (&buf[16 + AES_KEY_LENGTH..]).to_owned();

        Ok(Self {
            total_chunks,
            index,
            key,
            data,
        })
    }

    pub(crate) fn encrypt(&mut self, key: &[u8]) {
        let cipher = Cipher::aes_128_cbc();
        // let cipher = Cipher::aes_256_cbc();

        self.data = encrypt(cipher, key, Some(key), &self.data).unwrap();
    }

    pub(crate) fn decrypt(&mut self, key: &[u8]) {
        let cipher = Cipher::aes_128_cbc();
        // let cipher = Cipher::aes_256_cbc();

        self.data = decrypt(cipher, key, Some(key), &self.data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_key;

    use super::*;

    #[test]
    fn test_encryption() {
        let key = generate_key().unwrap();
        let orig_data = vec![12, 3, 5, 6, 78, 122, 34, 5, 6, 124, 34, 6, 7, 7];

        let mut ch = Chunk {
            total_chunks: 0,
            index: 0,
            key: key.to_owned(),
            data: orig_data.to_owned(),
        };

        ch.encrypt(&key);
        ch.decrypt(&key);

        assert_eq!(ch.data, orig_data);
    }
}
