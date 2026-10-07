//! Pure-Rust FIPS 180-4 SHA-256 and HMAC-SHA256 implementation
//!
//! Provides zero-dependency, deterministic cryptographic hashing and signature
//! verification for Zitera Lab packages.

#[allow(clippy::chunks_exact_to_as_chunks)]
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut state = [
        0x6a09e667u32,
        0xbb67ae85u32,
        0x3c6ef372u32,
        0xa54ff53au32,
        0x510e527fu32,
        0x9b05688cu32,
        0x1f83d9abu32,
        0x5be0cd19u32,
    ];

    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut buffer = Vec::with_capacity(data.len() + 72);
    buffer.extend_from_slice(data);
    buffer.push(0x80);

    while (buffer.len() % 64) != 56 {
        buffer.push(0x00);
    }
    buffer.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in buffer.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }

    let mut out = [0u8; 32];
    for (i, word) in state.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

pub fn sha256_hex(data: &[u8]) -> String {
    let hash = sha256(data);
    let mut hex = String::with_capacity(64);
    for b in hash {
        use std::fmt::Write;
        let _ = write!(hex, "{:02x}", b);
    }
    hex
}

pub fn sha256_file(path: &std::path::Path) -> Result<String, String> {
    let data =
        std::fs::read(path).map_err(|e| format!("Failed to read file for hashing: {}", e))?;
    Ok(sha256_hex(&data))
}

pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut k_block = [0u8; 64];
    if key.len() > 64 {
        let k_hash = sha256(key);
        k_block[..32].copy_from_slice(&k_hash);
    } else {
        k_block[..key.len()].copy_from_slice(key);
    }

    let mut i_key_pad = [0u8; 64];
    let mut o_key_pad = [0u8; 64];
    for i in 0..64 {
        i_key_pad[i] = k_block[i] ^ 0x36;
        o_key_pad[i] = k_block[i] ^ 0x5c;
    }

    let mut inner_data = Vec::with_capacity(64 + data.len());
    inner_data.extend_from_slice(&i_key_pad);
    inner_data.extend_from_slice(data);
    let inner_hash = sha256(&inner_data);

    let mut outer_data = Vec::with_capacity(64 + 32);
    outer_data.extend_from_slice(&o_key_pad);
    outer_data.extend_from_slice(&inner_hash);
    sha256(&outer_data)
}

pub fn hmac_sha256_hex(key: &[u8], data: &[u8]) -> String {
    let mac = hmac_sha256(key, data);
    let mut hex = String::with_capacity(64);
    for b in mac {
        use std::fmt::Write;
        let _ = write!(hex, "{:02x}", b);
    }
    hex
}

pub fn verify_hmac(key: &[u8], data: &[u8], expected_hex: &str) -> bool {
    let calculated = hmac_sha256_hex(key, data);
    // Constant-time comparison
    if calculated.len() != expected_hex.len() {
        return false;
    }
    let mut diff = 0u8;
    for (a, b) in calculated.bytes().zip(expected_hex.bytes()) {
        diff |= a ^ b;
    }
    diff == 0
}

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_vector() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            sha256_hex(b"The quick brown fox jumps over the lazy dog"),
            "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"
        );
    }

    #[test]
    fn test_hmac_sha256_known_vector() {
        let key = b"key";
        let data = b"The quick brown fox jumps over the lazy dog";
        let mac = hmac_sha256_hex(key, data);
        assert_eq!(
            mac,
            "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8"
        );
        assert!(verify_hmac(key, data, &mac));
        assert!(!verify_hmac(key, b"tampered data", &mac));
    }
}
