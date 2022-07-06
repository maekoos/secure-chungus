# Secure chungus (schungus)

A tool for splitting a file into a number of smaller chunks, where none of the original data can be retrieved from any of the chunks unless **all** chunks are supplied.

## How it works

The file is split into N different chunks, and each chunks gets its own unique encryption key. This key is then used to encrypt *all* chunks' data section, ordered by the chunk-index. To join the chunks and retrieve the original data, all chunks' data sections have to be decrypted with all chunks' key, in the opposite order of the encryption process. For encryption and decryption, 128-bit AES with CBC is used (with the same key and initialization vector).

Each chunk stores the total number of chunks required to unlock the original data, this chunk's index, a unique encryption key and it's data. The total overhead becomes 32 bytes per chunk.

```
[64-bit N-chunks] [64-bit chunk index] [128-bit signing key] [data]
```

# To Do:

 - [ ] Move to 256-bit AES
 - [ ] Print errors more nicely in main.rs (and update `impl fmt:: Display` for Error)
 - [ ] Support wildcard file paths
 - [ ] Allow user to select encryption standard and settings
 - [ ] Decide if we should add a file signature/magic to make it easier to identify the file and its version
 - [ ] Compile to wasm (+ switch from openssl to something more rusty)