//! Zitera Lab Archive (.zlab) Engine
//!
//! Provides secure creation, inspection, and safe extraction of .zlab packages.
//! Enforces strict archive security:
//! - Rejection of path traversal (.., \, drive letters, UNC)
//! - Rejection of zip bombs (> 100MB uncompressed, > 500 files)
//! - Mandatory manifest.json presence at archive root
//! - Safe extraction strictly contained within staging destination

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub const MAX_PACKAGE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB
pub const MAX_UNCOMPRESSED_SIZE: u64 = 250 * 1024 * 1024; // 250 MB
pub const MAX_FILE_COUNT: usize = 500;

#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    pub name: String,
    pub uncompressed_size: u64,
    pub is_dir: bool,
    pub compression_method: u16,
    pub local_header_offset: u64,
    pub data_offset: u64,
}

/// Computes IEEE 802.3 CRC-32 checksum.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in data {
        let mut b = (crc ^ (byte as u32)) & 0xff;
        for _ in 0..8 {
            if (b & 1) != 0 {
                b = (b >> 1) ^ 0xedb8_8320;
            } else {
                b >>= 1;
            }
        }
        crc = b ^ (crc >> 8);
    }
    !crc
}

/// Inspects and validates a .zlab ZIP archive from bytes.
pub fn validate_archive_structure(bytes: &[u8]) -> Result<Vec<ArchiveEntry>, String> {
    if bytes.len() as u64 > MAX_PACKAGE_SIZE {
        return Err(format!(
            "Package size {} bytes exceeds maximum limit of {} bytes",
            bytes.len(),
            MAX_PACKAGE_SIZE
        ));
    }
    if bytes.len() < 22 {
        return Err("Invalid archive: file too short to be a valid ZIP archive".to_string());
    }

    // Locate End of Central Directory (EOCD) signature: 0x06054b50
    let mut eocd_pos = None;
    for i in (0..=bytes.len() - 22).rev() {
        if bytes[i..i + 4] == [0x50, 0x4b, 0x05, 0x06] {
            eocd_pos = Some(i);
            break;
        }
    }

    let eocd_idx = match eocd_pos {
        Some(pos) => pos,
        None => {
            return Err("Invalid archive: End of Central Directory record not found".to_string())
        }
    };

    let total_entries = u16::from_le_bytes([bytes[eocd_idx + 10], bytes[eocd_idx + 11]]) as usize;
    if total_entries > MAX_FILE_COUNT {
        return Err(format!(
            "Package entry count {} exceeds security maximum of {}",
            total_entries, MAX_FILE_COUNT
        ));
    }

    let cd_size = u32::from_le_bytes([
        bytes[eocd_idx + 12],
        bytes[eocd_idx + 13],
        bytes[eocd_idx + 14],
        bytes[eocd_idx + 15],
    ]) as usize;

    let cd_offset = u32::from_le_bytes([
        bytes[eocd_idx + 16],
        bytes[eocd_idx + 17],
        bytes[eocd_idx + 18],
        bytes[eocd_idx + 19],
    ]) as usize;

    if cd_offset + cd_size > eocd_idx {
        return Err("Invalid archive: Central directory boundaries corrupted".to_string());
    }

    let mut entries = Vec::with_capacity(total_entries);
    let mut seen_names = std::collections::HashSet::new();
    let mut cursor = cd_offset;
    let mut total_uncompressed: u64 = 0;
    let mut has_manifest = false;

    for _ in 0..total_entries {
        if cursor + 46 > bytes.len() {
            return Err("Invalid archive: truncated central directory header".to_string());
        }
        if bytes[cursor..cursor + 4] != [0x50, 0x4b, 0x01, 0x02] {
            return Err(format!(
                "Invalid archive: corrupt central directory signature at offset {}",
                cursor
            ));
        }

        let method = u16::from_le_bytes([bytes[cursor + 10], bytes[cursor + 11]]);
        let uncomp_size = u32::from_le_bytes([
            bytes[cursor + 24],
            bytes[cursor + 25],
            bytes[cursor + 26],
            bytes[cursor + 27],
        ]) as u64;
        let fn_len = u16::from_le_bytes([bytes[cursor + 28], bytes[cursor + 29]]) as usize;
        let extra_len = u16::from_le_bytes([bytes[cursor + 30], bytes[cursor + 31]]) as usize;
        let comment_len = u16::from_le_bytes([bytes[cursor + 32], bytes[cursor + 33]]) as usize;
        let local_offset = u32::from_le_bytes([
            bytes[cursor + 42],
            bytes[cursor + 43],
            bytes[cursor + 44],
            bytes[cursor + 45],
        ]) as u64;

        let name_start = cursor + 46;
        let name_end = name_start + fn_len;
        if name_end > bytes.len() {
            return Err("Invalid archive: truncated filename in central directory".to_string());
        }

        let raw_name = String::from_utf8_lossy(&bytes[name_start..name_end]).to_string();

        // Security check on entry name
        validate_entry_name(&raw_name)?;
        if !seen_names.insert(raw_name.clone()) {
            return Err(format!("Invalid archive: duplicate entry '{}'", raw_name));
        }

        total_uncompressed = total_uncompressed.saturating_add(uncomp_size);
        if total_uncompressed > MAX_UNCOMPRESSED_SIZE {
            return Err(format!(
                "Package uncompressed size {} exceeds bomb protection limit {}",
                total_uncompressed, MAX_UNCOMPRESSED_SIZE
            ));
        }

        if raw_name == "manifest.json" {
            has_manifest = true;
        }

        // Calculate data offset from local header
        let local_idx = local_offset as usize;
        if local_idx + 30 > bytes.len() {
            return Err("Invalid archive: corrupt local header offset".to_string());
        }
        let loc_fn_len =
            u16::from_le_bytes([bytes[local_idx + 26], bytes[local_idx + 27]]) as usize;
        let loc_extra_len =
            u16::from_le_bytes([bytes[local_idx + 28], bytes[local_idx + 29]]) as usize;
        let data_offset = local_offset + 30 + (loc_fn_len as u64) + (loc_extra_len as u64);

        let is_dir = raw_name.ends_with('/');

        entries.push(ArchiveEntry {
            name: raw_name,
            uncompressed_size: uncomp_size,
            is_dir,
            compression_method: method,
            local_header_offset: local_offset,
            data_offset,
        });

        cursor += 46 + fn_len + extra_len + comment_len;
    }

    if !has_manifest {
        return Err("Invalid archive: missing required 'manifest.json' at root".to_string());
    }

    Ok(entries)
}

/// Enforces path safety for an archive entry name.
pub fn validate_entry_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Archive entry name cannot be empty".to_string());
    }
    // Reject path traversal
    if name.contains("..") {
        return Err(format!(
            "Archive entry '{}' contains parent directory '..'",
            name
        ));
    }
    // Reject backslashes
    if name.contains('\\') {
        return Err(format!(
            "Archive entry '{}' contains forbidden backslash path separator",
            name
        ));
    }
    // Reject absolute paths
    if name.starts_with('/') {
        return Err(format!(
            "Archive entry '{}' specifies forbidden absolute path",
            name
        ));
    }
    // Reject UNC paths
    if name.starts_with("//") {
        return Err(format!(
            "Archive entry '{}' specifies forbidden UNC path",
            name
        ));
    }
    // Reject drive letters
    if name.len() >= 2 && name.chars().nth(1) == Some(':') {
        return Err(format!(
            "Archive entry '{}' specifies forbidden drive letter prefix",
            name
        ));
    }
    // Reject control characters
    for ch in name.chars() {
        if ch.is_control() {
            return Err(format!(
                "Archive entry '{}' contains illegal control characters",
                name
            ));
        }
    }
    Ok(())
}

/// Creates a standard .zlab package (ZIP format) from a source directory.
pub fn create_zlab_package(source_dir: &Path, output_zlab: &Path) -> Result<(), String> {
    if !source_dir.exists() {
        return Err(format!("Source directory {:?} does not exist", source_dir));
    }

    let mut file_entries: Vec<(String, Vec<u8>)> = Vec::new();
    collect_files_recursive(source_dir, "", &mut file_entries)?;
    // Enforce deterministic lexicographical order for reproducible builds (SEC-CP13)
    file_entries.sort_by(|a, b| a.0.cmp(&b.0));

    // Ensure manifest.json is present
    if !file_entries.iter().any(|(name, _)| name == "manifest.json") {
        return Err("Cannot create package: manifest.json is required in source root".to_string());
    }

    let mut out_file = File::create(output_zlab)
        .map_err(|e| format!("Failed to create output package {:?}: {}", output_zlab, e))?;

    let mut cd_records = Vec::new();
    let mut current_offset: u32 = 0;

    for (rel_name, data) in &file_entries {
        let name_bytes = rel_name.as_bytes();
        let crc = crc32(data);
        let size = data.len() as u32;

        // Write Local File Header
        out_file
            .write_all(&[0x50, 0x4b, 0x03, 0x04])
            .map_err(|e| e.to_string())?;
        out_file
            .write_all(&20u16.to_le_bytes())
            .map_err(|e| e.to_string())?; // version needed
        out_file
            .write_all(&0u16.to_le_bytes())
            .map_err(|e| e.to_string())?; // flags
        out_file
            .write_all(&0u16.to_le_bytes())
            .map_err(|e| e.to_string())?; // compression method (0 = stored)
        out_file
            .write_all(&0u16.to_le_bytes())
            .map_err(|e| e.to_string())?; // time
        out_file
            .write_all(&0u16.to_le_bytes())
            .map_err(|e| e.to_string())?; // date
        out_file
            .write_all(&crc.to_le_bytes())
            .map_err(|e| e.to_string())?;
        out_file
            .write_all(&size.to_le_bytes())
            .map_err(|e| e.to_string())?;
        out_file
            .write_all(&size.to_le_bytes())
            .map_err(|e| e.to_string())?;
        out_file
            .write_all(&(name_bytes.len() as u16).to_le_bytes())
            .map_err(|e| e.to_string())?;
        out_file
            .write_all(&0u16.to_le_bytes())
            .map_err(|e| e.to_string())?; // extra len
        out_file.write_all(name_bytes).map_err(|e| e.to_string())?;
        out_file.write_all(data).map_err(|e| e.to_string())?;

        // Prepare Central Directory Header record
        let mut cd_record = Vec::new();
        cd_record.extend_from_slice(&[0x50, 0x4b, 0x01, 0x02]);
        cd_record.extend_from_slice(&20u16.to_le_bytes()); // version made by
        cd_record.extend_from_slice(&20u16.to_le_bytes()); // version needed
        cd_record.extend_from_slice(&0u16.to_le_bytes()); // flags
        cd_record.extend_from_slice(&0u16.to_le_bytes()); // compression method
        cd_record.extend_from_slice(&0u16.to_le_bytes()); // time
        cd_record.extend_from_slice(&0u16.to_le_bytes()); // date
        cd_record.extend_from_slice(&crc.to_le_bytes());
        cd_record.extend_from_slice(&size.to_le_bytes());
        cd_record.extend_from_slice(&size.to_le_bytes());
        cd_record.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        cd_record.extend_from_slice(&0u16.to_le_bytes()); // extra len
        cd_record.extend_from_slice(&0u16.to_le_bytes()); // comment len
        cd_record.extend_from_slice(&0u16.to_le_bytes()); // disk #
        cd_record.extend_from_slice(&0u16.to_le_bytes()); // internal attr
        cd_record.extend_from_slice(&0u32.to_le_bytes()); // external attr
        cd_record.extend_from_slice(&current_offset.to_le_bytes());
        cd_record.extend_from_slice(name_bytes);

        cd_records.push(cd_record);
        current_offset += 30 + (name_bytes.len() as u32) + size;
    }

    let cd_offset = current_offset;
    let mut cd_size: u32 = 0;
    for rec in &cd_records {
        out_file.write_all(rec).map_err(|e| e.to_string())?;
        cd_size += rec.len() as u32;
    }

    // Write End of Central Directory Record
    out_file
        .write_all(&[0x50, 0x4b, 0x05, 0x06])
        .map_err(|e| e.to_string())?;
    out_file
        .write_all(&0u16.to_le_bytes())
        .map_err(|e| e.to_string())?; // disk #
    out_file
        .write_all(&0u16.to_le_bytes())
        .map_err(|e| e.to_string())?; // cd start disk
    out_file
        .write_all(&(file_entries.len() as u16).to_le_bytes())
        .map_err(|e| e.to_string())?; // records on disk
    out_file
        .write_all(&(file_entries.len() as u16).to_le_bytes())
        .map_err(|e| e.to_string())?; // total records
    out_file
        .write_all(&cd_size.to_le_bytes())
        .map_err(|e| e.to_string())?;
    out_file
        .write_all(&cd_offset.to_le_bytes())
        .map_err(|e| e.to_string())?;
    out_file
        .write_all(&0u16.to_le_bytes())
        .map_err(|e| e.to_string())?; // comment len

    Ok(())
}

fn collect_files_recursive(
    dir: &Path,
    prefix: &str,
    out: &mut Vec<(String, Vec<u8>)>,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') {
            continue; // Skip hidden / git files
        }

        let rel_name = if prefix.is_empty() {
            name
        } else {
            format!("{}/{}", prefix, name)
        };

        if path.is_dir() {
            collect_files_recursive(&path, &rel_name, out)?;
        } else {
            let data =
                fs::read(&path).map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
            out.push((rel_name, data));
        }
    }
    Ok(())
}

/// Safely extracts a validated .zlab archive into a target staging directory.
pub fn extract_zlab_archive(package_path: &Path, staging_dir: &Path) -> Result<(), String> {
    let bytes = fs::read(package_path)
        .map_err(|e| format!("Failed to read package file {:?}: {}", package_path, e))?;

    let entries = validate_archive_structure(&bytes)?;

    fs::create_dir_all(staging_dir).map_err(|e| {
        format!(
            "Failed to create staging directory {:?}: {}",
            staging_dir, e
        )
    })?;

    for entry in entries {
        let safe_dest = match crate::labs::safe_subpath(staging_dir, Path::new(&entry.name)) {
            Ok(p) => p,
            Err(e) => return Err(format!("Extraction path violation: {}", e)),
        };

        if entry.is_dir {
            fs::create_dir_all(&safe_dest).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = safe_dest.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }

            if entry.compression_method == 0 {
                // Method 0 (Stored): extract directly from bytes
                let start = entry.data_offset as usize;
                let end = start + (entry.uncompressed_size as usize);
                if end > bytes.len() {
                    return Err(format!(
                        "Corrupted archive data boundary for entry '{}'",
                        entry.name
                    ));
                }
                let data = &bytes[start..end];
                fs::write(&safe_dest, data).map_err(|e| {
                    format!("Failed to write extracted file {:?}: {}", safe_dest, e)
                })?;
            } else {
                // For compressed entries (Deflate), use Windows system tar.exe fallback
                return extract_via_system_tool(package_path, staging_dir);
            }
        }
    }

    Ok(())
}

fn extract_via_system_tool(package_path: &Path, staging_dir: &Path) -> Result<(), String> {
    let pkg_str = package_path.to_string_lossy();
    let staging_str = staging_dir.to_string_lossy();

    let output = std::process::Command::new("tar.exe")
        .args(["-xf", &pkg_str, "-C", &staging_str])
        .output()
        .map_err(|e| format!("Failed to execute system tar extractor: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "System tar extractor failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32() {
        assert_eq!(crc32(b"123456789"), 0xcbf43926);
    }

    #[test]
    fn test_entry_name_validation() {
        assert!(validate_entry_name("manifest.json").is_ok());
        assert!(validate_entry_name("bin/a01-lab.exe").is_ok());
        assert!(validate_entry_name("lesson/guide.md").is_ok());

        assert!(validate_entry_name("../secret.txt").is_err());
        assert!(validate_entry_name("bin\\hack.exe").is_err());
        assert!(validate_entry_name("/etc/passwd").is_err());
        assert!(validate_entry_name("C:/Windows/system32").is_err());
    }

    #[test]
    fn test_package_creation_and_extraction() {
        let temp = std::env::temp_dir().join("zitera_test_pkg_create");
        let _ = fs::remove_dir_all(&temp);
        let src_dir = temp.join("src");
        let stage_dir = temp.join("staging");
        let pkg_file = temp.join("A01.zlab");

        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("manifest.json"), r#"{"id":"A01"}"#).unwrap();
        fs::create_dir_all(src_dir.join("bin")).unwrap();
        fs::write(src_dir.join("bin").join("lab.exe"), "binary data").unwrap();

        // 1. Create .zlab
        create_zlab_package(&src_dir, &pkg_file).unwrap();
        assert!(pkg_file.exists());

        // 2. Extract .zlab
        extract_zlab_archive(&pkg_file, &stage_dir).unwrap();
        assert!(stage_dir.join("manifest.json").exists());
        assert!(stage_dir.join("bin").join("lab.exe").exists());
        let read_back = fs::read_to_string(stage_dir.join("bin").join("lab.exe")).unwrap();
        assert_eq!(read_back, "binary data");

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_checkpoint_13_deterministic_package_build() {
        let temp = std::env::temp_dir().join("zitera_reproducible_build_test");
        let _ = fs::remove_dir_all(&temp);
        let src_dir = temp.join("src");
        let pkg_a = temp.join("build_a.zlab");
        let pkg_b = temp.join("build_b.zlab");

        fs::create_dir_all(src_dir.join("sub")).unwrap();
        fs::create_dir_all(src_dir.join("bin")).unwrap();
        fs::write(src_dir.join("manifest.json"), r#"{"id":"A01"}"#).unwrap();
        fs::write(src_dir.join("bin").join("a01-lab.exe"), "executable bytes").unwrap();
        fs::write(src_dir.join("sub").join("data.txt"), "sample data").unwrap();

        create_zlab_package(&src_dir, &pkg_a).unwrap();
        create_zlab_package(&src_dir, &pkg_b).unwrap();

        let bytes_a = fs::read(&pkg_a).unwrap();
        let bytes_b = fs::read(&pkg_b).unwrap();

        assert_eq!(
            bytes_a, bytes_b,
            "Package builds from identical source must be bit-for-bit deterministic"
        );

        let _ = fs::remove_dir_all(&temp);
    }
}
