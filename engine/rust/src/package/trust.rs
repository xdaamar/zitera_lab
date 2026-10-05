//! Zitera Cryptographic Trust and Ed25519 Signature Verification
//!
//! Provides deterministic canonical payload construction, Ed25519 signing (for CI/dev),
//! and client-side public-key verification for packages and catalogs.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

pub const SIGNATURE_HEX_LEN: usize = 128; // 64 bytes * 2
pub const PUBLIC_KEY_HEX_LEN: usize = 64; // 32 bytes * 2

/// Embedded development public key (derived from deterministic seed for testing/dev builds)
pub const DEV_PUBLIC_KEY_HEX: &str =
    "4c21e05aa2470e72b6f83bc3c1c97380947f8ba2a16b6d1a8054de4e26d444b7";

/// Embedded production release public key (Active release authority)
pub const RELEASE_PUBLIC_KEY_HEX: &str =
    "ec085cf5579998d7936a619213197f2258832a8ddcc1645e7f91ff39ee312d8a";

/// Deterministic dev private key seed (Used ONLY in testing and local package generation, never in production client)
pub const DEV_PRIVATE_KEY_SEED: [u8; 32] = [
    0x5a, 0x69, 0x74, 0x65, 0x72, 0x61, 0x44, 0x65, // "ZiteraDe"
    0x76, 0x53, 0x65, 0x65, 0x64, 0x50, 0x68, 0x61, // "vSeedPha"
    0x73, 0x65, 0x31, 0x38, 0x54, 0x72, 0x75, 0x73, // "se18Trus"
    0x74, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x56, 0x31, // "tChainV1"
];

/// Encodes raw bytes into a lowercase hexadecimal string.
pub fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Decodes a hexadecimal string into a byte vector.
pub fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.trim();
    if !hex.len().is_multiple_of(2) {
        return Err("Hex string has odd length".to_string());
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        let byte = u8::from_str_radix(byte_str, 16)
            .map_err(|e| format!("Invalid hex character in '{}': {}", byte_str, e))?;
        bytes.push(byte);
    }
    Ok(bytes)
}

/// Builds the canonical deterministic representation of a package payload.
///
/// Must be sorted lexicographically by field name, newline-separated,
/// with normalized field values to ensure cross-platform reproducibility.
#[allow(clippy::too_many_arguments)]
pub fn build_canonical_package_payload(
    id: &str,
    version: &str,
    security_version: u32,
    minimum_core_version: &str,
    runtime: &str,
    architecture: &str,
    entrypoint: &str,
    content_digest: &str,
) -> String {
    let normalized_entrypoint = entrypoint.replace('\\', "/");
    format!(
        "architecture:{}\ncontent_digest:{}\nentrypoint:{}\nid:{}\nminimum_core_version:{}\nruntime:{}\nsecurity_version:{}\nversion:{}",
        architecture.trim().to_lowercase(),
        content_digest.trim().to_lowercase(),
        normalized_entrypoint.trim(),
        id.trim().to_uppercase(),
        minimum_core_version.trim(),
        runtime.trim(),
        security_version,
        version.trim()
    )
}

/// Signs a canonical payload using an Ed25519 32-byte private key seed.
/// Returns a 128-character lowercase hexadecimal signature.
pub fn sign_payload(canonical_payload: &str, private_key_seed: &[u8; 32]) -> String {
    let signing_key = SigningKey::from_bytes(private_key_seed);
    let signature = signing_key.sign(canonical_payload.as_bytes());
    hex_encode(&signature.to_bytes())
}

/// Verifies an Ed25519 signature against a canonical payload using a 32-byte public key.
pub fn verify_signature_bytes(
    canonical_payload: &str,
    signature_hex: &str,
    public_key_bytes: &[u8; 32],
) -> Result<(), String> {
    let sig_str = signature_hex.trim();
    if sig_str.is_empty() {
        return Err("Signature is empty".to_string());
    }
    if sig_str.len() != SIGNATURE_HEX_LEN {
        return Err(format!(
            "Invalid signature length: expected {} hex chars, got {}",
            SIGNATURE_HEX_LEN,
            sig_str.len()
        ));
    }

    let sig_bytes_vec = hex_decode(sig_str)?;
    let mut sig_bytes = [0u8; 64];
    sig_bytes.copy_from_slice(&sig_bytes_vec);

    let signature = Signature::from_bytes(&sig_bytes);
    let verifying_key = VerifyingKey::from_bytes(public_key_bytes)
        .map_err(|e| format!("Invalid public key: {}", e))?;

    verifying_key
        .verify(canonical_payload.as_bytes(), &signature)
        .map_err(|e| format!("Cryptographic signature verification failed: {}", e))
}

/// Verifies an Ed25519 signature against a canonical payload using a hex-encoded public key.
pub fn verify_signature(
    canonical_payload: &str,
    signature_hex: &str,
    public_key_hex: &str,
) -> Result<(), String> {
    let pk_hex = public_key_hex.trim();
    if pk_hex.len() != PUBLIC_KEY_HEX_LEN {
        return Err(format!(
            "Invalid public key length: expected {} hex chars, got {}",
            PUBLIC_KEY_HEX_LEN,
            pk_hex.len()
        ));
    }

    let pk_vec = hex_decode(pk_hex)?;
    let mut pk_bytes = [0u8; 32];
    pk_bytes.copy_from_slice(&pk_vec);

    verify_signature_bytes(canonical_payload, signature_hex, &pk_bytes)
}

/// Verifies a canonical payload against the set of trusted public keys.
pub fn verify_with_trusted_keys(
    canonical_payload: &str,
    signature_hex: &str,
    trusted_public_keys: &[&str],
) -> Result<String, String> {
    let sig_str = signature_hex.trim();
    if sig_str.is_empty() {
        return Err("Signature is empty".to_string());
    }
    if sig_str.len() != SIGNATURE_HEX_LEN {
        return Err(format!(
            "Invalid signature length: expected {} hex chars, got {}",
            SIGNATURE_HEX_LEN,
            sig_str.len()
        ));
    }

    for (i, &pk_hex) in trusted_public_keys.iter().enumerate() {
        if verify_signature(canonical_payload, sig_str, pk_hex).is_ok() {
            return Ok(format!("Key #{} ({})", i, &pk_hex[0..8]));
        }
    }

    Err("Signature does not match any trusted root public key".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_payload_determinism() {
        let p1 = build_canonical_package_payload(
            "A01",
            "1.0.1",
            1,
            "2.0.0",
            "native_sandboxed",
            "x86_64",
            "bin\\a01-lab.exe",
            "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
        );

        let p2 = build_canonical_package_payload(
            " a01 ",
            "1.0.1 ",
            1,
            "2.0.0",
            "native_sandboxed",
            "X86_64",
            "bin/a01-lab.exe",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        );

        assert_eq!(
            p1, p2,
            "Canonical representation must normalize casing, slashes, and whitespace"
        );
        assert!(p1.contains("id:A01"));
        assert!(p1.contains("entrypoint:bin/a01-lab.exe"));
        assert!(p1.contains("architecture:x86_64"));
    }

    #[test]
    fn test_checkpoint_2_asymmetric_signature_matrix() {
        let payload = build_canonical_package_payload(
            "A01",
            "1.0.1",
            1,
            "2.0.0",
            "native_sandboxed",
            "x86_64",
            "bin/a01-lab.exe",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        );

        // Derive dev keypair
        let signing_key = SigningKey::from_bytes(&DEV_PRIVATE_KEY_SEED);
        let verifying_key = signing_key.verifying_key();
        let pubkey_hex = hex_encode(&verifying_key.to_bytes());

        // 1. VALID SIGNATURE -> PASS
        let sig = sign_payload(&payload, &DEV_PRIVATE_KEY_SEED);
        assert_eq!(sig.len(), 128);
        assert!(verify_signature(&payload, &sig, &pubkey_hex).is_ok());
        assert!(verify_with_trusted_keys(&payload, &sig, &[&pubkey_hex]).is_ok());

        // 2. MODIFIED CONTENT -> FAIL
        let modified_content_payload = build_canonical_package_payload(
            "A01",
            "1.0.1",
            1,
            "2.0.0",
            "native_sandboxed",
            "x86_64",
            "bin/a01-lab.exe",
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        );
        let err_content = verify_signature(&modified_content_payload, &sig, &pubkey_hex);
        assert!(
            err_content.is_err(),
            "Modified content digest must fail verification"
        );

        // 3. MODIFIED VERSION -> FAIL
        let modified_version_payload = build_canonical_package_payload(
            "A01",
            "1.0.2",
            1,
            "2.0.0",
            "native_sandboxed",
            "x86_64",
            "bin/a01-lab.exe",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        );
        let err_version = verify_signature(&modified_version_payload, &sig, &pubkey_hex);
        assert!(
            err_version.is_err(),
            "Modified version must fail verification"
        );

        // 4. MODIFIED LAB ID -> FAIL
        let modified_id_payload = build_canonical_package_payload(
            "A02",
            "1.0.1",
            1,
            "2.0.0",
            "native_sandboxed",
            "x86_64",
            "bin/a01-lab.exe",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        );
        let err_id = verify_signature(&modified_id_payload, &sig, &pubkey_hex);
        assert!(err_id.is_err(), "Modified lab ID must fail verification");

        // 5. MODIFIED HASH -> FAIL
        let mut tampered_sig = sig.clone();
        tampered_sig.replace_range(0..2, "ff");
        if tampered_sig == sig {
            tampered_sig.replace_range(0..2, "00");
        }
        let err_hash = verify_signature(&payload, &tampered_sig, &pubkey_hex);
        assert!(
            err_hash.is_err(),
            "Tampered signature bytes must fail verification"
        );

        // 6. WRONG PUBLIC KEY -> FAIL
        let wrong_seed = [0x99u8; 32];
        let wrong_signing_key = SigningKey::from_bytes(&wrong_seed);
        let wrong_pubkey_hex = hex_encode(&wrong_signing_key.verifying_key().to_bytes());
        let err_wrong_key = verify_signature(&payload, &sig, &wrong_pubkey_hex);
        assert!(
            err_wrong_key.is_err(),
            "Wrong public key must fail verification"
        );

        // 7. TRUNCATED SIGNATURE -> FAIL
        let truncated_sig = &sig[0..64];
        let err_trunc = verify_signature(&payload, truncated_sig, &pubkey_hex);
        assert!(
            err_trunc.is_err(),
            "Truncated signature must fail verification"
        );
        assert!(err_trunc.unwrap_err().contains("Invalid signature length"));

        // 8. CORRUPT SIGNATURE -> FAIL
        let corrupt_sig = format!("{}!!invalid_hex!!", &sig[0..110]);
        let err_corrupt = verify_signature(&payload, &corrupt_sig, &pubkey_hex);
        assert!(
            err_corrupt.is_err(),
            "Corrupt non-hex signature must fail verification"
        );

        // 9. EMPTY SIGNATURE -> FAIL
        let empty_sig = "";
        let err_empty = verify_signature(&payload, empty_sig, &pubkey_hex);
        assert!(err_empty.is_err(), "Empty signature must fail verification");
        assert!(err_empty.unwrap_err().contains("empty"));
    }
}
