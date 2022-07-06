use openssl::rand::rand_bytes;

const AES_KEY_LENGTH: usize = 16;
type AesKey = [u8; AES_KEY_LENGTH];

mod chunk;
mod error;
pub use chunk::Chunk;
pub use error::Error;

pub fn combine_chunks(chunks: Vec<Chunk>) -> Result<Vec<u8>, Error> {
    if chunks.len() as u64 != chunks[0].total_chunks {
        return Err(Error::WrongNumberOfChunks);
    }

    // This clone is necessary because we need chunks to be mutable when decrypting
    let mut chunks = chunks.clone();
    chunks.sort_by_key(|c| c.index);

    for key in chunks.iter().map(|x| x.key).rev().collect::<Vec<AesKey>>() {
        // For each chunk, decrypt all chunks' data with the current chunk's key
        for chunk in &mut chunks {
            chunk.decrypt(&key);
        }
    }

    // TODO Maybe start this with a capacity
    let mut out: Vec<u8> = Vec::new();
    for chunk in chunks {
        out.extend(chunk.data.iter());
    }

    Ok(out)
}

pub fn generate_chunks(n_chunks: u64, input: &[u8]) -> Result<Vec<Chunk>, Error> {
    let keys = generate_keys(n_chunks)?;

    // Split the input into multiple smaller chunks
    let mut chunks: Vec<Chunk> = split_file(n_chunks, &input)?
        .into_iter()
        .enumerate()
        .map(|(idx, data)| Chunk {
            total_chunks: n_chunks as u64,
            index: idx as u64,
            key: keys[idx],
            data: data.to_owned(),
        })
        .collect();

    for key in keys {
        for ch in &mut chunks {
            ch.encrypt(&key);
        }
    }

    Ok(chunks)
}

fn generate_keys(n: u64) -> Result<Vec<AesKey>, Error> {
    let mut keys = Vec::new();
    for _ in 0..n {
        keys.push(generate_key()?)
    }
    Ok(keys)
}

fn generate_key() -> Result<AesKey, Error> {
    let mut buf: AesKey = [0_u8; AES_KEY_LENGTH];
    rand_bytes(&mut buf).map_err(|_| Error::RandBytesFailure)?;
    Ok(buf)
}

fn split_file<'a>(n_chunks: u64, input: &'a [u8]) -> Result<Vec<&'a [u8]>, Error> {
    // TODO: Handle this by padding or duplicating data?
    if (input.len() as u64) < n_chunks {
        return Err(Error::InputSmallerThanNumberOfChunks);
    }

    let mut out = vec![];
    let chunk_size = input.len() as u64 / n_chunks;

    let mut buffer_idx: usize = 0;
    for i in 0..n_chunks {
        let end_idx = if i == n_chunks - 1 {
            // The last chunk may contain more bytes if necessary
            input.len()
        } else {
            buffer_idx + chunk_size as usize
        };

        let chunk = &input[buffer_idx..end_idx];
        buffer_idx += chunk.len();
        out.push(chunk);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_from_to_buf() {
        let ch = Chunk {
            total_chunks: 100,
            index: 10,
            key: [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
                15,
                // 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            ],
            data: vec![],
        };

        let buf = ch.to_buf();
        let ch2 = Chunk::from_buf(&buf).unwrap();

        assert_eq!(ch, ch2);
    }

    #[test]
    fn test_openssl_aes() {
        use openssl::symm::{encrypt, Cipher};

        let cipher = Cipher::aes_128_cbc();
        let data = b"Some Crypto Text";
        let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
        let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
        let ciphertext = encrypt(cipher, key, Some(iv), data).unwrap();

        assert_eq!(
    b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\x87\x4D\
      \xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1",
    &ciphertext[..]);
    }

    #[test]
    fn test_chunking() {
        let buffer = b"asdfkjasdlufis14o\n2630\t123askdj\x10124\xAA";
        let chunks = generate_chunks(4, buffer).unwrap();
        let orig = combine_chunks(chunks).unwrap();
        assert_eq!(orig, buffer);
    }
}
