//! Zitera Terminal Sandboxed Virtual Filesystem
//!
//! Enforces a strict logical directory tree rooted inside the lab environment.
//! Completely shields the host OS filesystem (C:\, C:\Windows, UNC, path traversal).

use super::parser::TerminalError;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone)]
pub struct TerminalFilesystem {
    root_dir: PathBuf,
    current_virtual_dir: String, // Always begins with "/" e.g. "/" or "/src"
}

impl TerminalFilesystem {
    /// Creates a new sandboxed filesystem rooted at `physical_root`.
    pub fn new(physical_root: PathBuf) -> Self {
        let canonical_root = physical_root
            .canonicalize()
            .unwrap_or_else(|_| physical_root.clone());
        Self {
            root_dir: canonical_root,
            current_virtual_dir: "/".to_string(),
        }
    }

    /// Returns the current logical working directory (e.g. "/" or "/subdir").
    pub fn current_virtual_dir(&self) -> &str {
        &self.current_virtual_dir
    }

    /// Normalizes virtual path segments (resolving "." and "..") strictly within "/".
    pub fn normalize_virtual_path(&self, raw_virtual: &str) -> String {
        let parts = raw_virtual.split('/');
        let mut stack: Vec<&str> = Vec::new();

        for part in parts {
            if part.is_empty() || part == "." {
                continue;
            } else if part == ".." {
                stack.pop();
            } else {
                stack.push(part);
            }
        }

        if stack.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", stack.join("/"))
        }
    }

    /// Resolves a user-supplied path into a validated physical path strictly within root_dir.
    pub fn resolve_path(&self, user_path: &str) -> Result<PathBuf, TerminalError> {
        let trimmed = user_path.trim();

        // 1. Strict rejection of Windows drive letters, UNC, and backslashes
        if trimmed.contains('\\') {
            return Err(TerminalError::PathTraversalAttempt(user_path.to_string()));
        }
        if trimmed.len() >= 2 && trimmed.chars().nth(1) == Some(':') {
            return Err(TerminalError::PathTraversalAttempt(user_path.to_string()));
        }
        if trimmed.starts_with("//") {
            return Err(TerminalError::PathTraversalAttempt(user_path.to_string()));
        }

        // 2. Compute candidate virtual path
        let candidate_virtual = if trimmed.starts_with('/') {
            self.normalize_virtual_path(trimmed)
        } else if self.current_virtual_dir == "/" {
            self.normalize_virtual_path(&format!("/{}", trimmed))
        } else {
            self.normalize_virtual_path(&format!("{}/{}", self.current_virtual_dir, trimmed))
        };

        // 3. Map virtual path to physical path under root_dir
        let relative_path = candidate_virtual.trim_start_matches('/');
        let candidate_physical = if relative_path.is_empty() {
            self.root_dir.clone()
        } else {
            self.root_dir.join(relative_path)
        };

        // 4. Double check boundary via components to prevent escaping
        for comp in candidate_physical.components() {
            if matches!(comp, Component::ParentDir) {
                return Err(TerminalError::PathTraversalAttempt(user_path.to_string()));
            }
        }

        if let Ok(canon) = candidate_physical.canonicalize() {
            if !canon.starts_with(&self.root_dir) {
                return Err(TerminalError::PathTraversalAttempt(user_path.to_string()));
            }
        }

        Ok(candidate_physical)
    }

    /// Translates a physical path back to a logical virtual path string.
    pub fn to_virtual_path(&self, physical: &Path) -> Result<String, TerminalError> {
        let rel = physical.strip_prefix(&self.root_dir).map_err(|_| {
            TerminalError::PathTraversalAttempt(physical.to_string_lossy().to_string())
        })?;

        let mut vpath = String::new();
        vpath.push('/');
        let components: Vec<String> = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();
        vpath.push_str(&components.join("/"));
        let trimmed = vpath.trim_end_matches('/');
        if trimmed.is_empty() {
            Ok("/".to_string())
        } else {
            Ok(trimmed.to_string())
        }
    }

    /// Changes directory within the virtual tree.
    pub fn change_dir(&mut self, target: &str) -> Result<(), TerminalError> {
        let physical = self.resolve_path(target)?;
        if !physical.exists() {
            return Err(TerminalError::FileNotFound(target.to_string()));
        }
        if !physical.is_dir() {
            return Err(TerminalError::NotADirectory(target.to_string()));
        }

        let new_virt = if target.starts_with('/') {
            self.normalize_virtual_path(target)
        } else if self.current_virtual_dir == "/" {
            self.normalize_virtual_path(&format!("/{}", target))
        } else {
            self.normalize_virtual_path(&format!("{}/{}", self.current_virtual_dir, target))
        };

        self.current_virtual_dir = new_virt;
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Result<String, TerminalError> {
        let physical = self.resolve_path(path)?;
        if !physical.exists() {
            return Err(TerminalError::FileNotFound(path.to_string()));
        }
        if physical.is_dir() {
            return Err(TerminalError::IsADirectory(path.to_string()));
        }
        fs::read_to_string(&physical)
            .map_err(|e| TerminalError::ExecutionError(format!("read error: {}", e)))
    }

    pub fn write_file(&self, path: &str, content: &str) -> Result<(), TerminalError> {
        let physical = self.resolve_path(path)?;
        if let Some(parent) = physical.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(&physical, content)
            .map_err(|e| TerminalError::ExecutionError(format!("write error: {}", e)))
    }

    pub fn make_dir(&self, path: &str) -> Result<(), TerminalError> {
        let physical = self.resolve_path(path)?;
        fs::create_dir_all(&physical)
            .map_err(|e| TerminalError::ExecutionError(format!("mkdir error: {}", e)))
    }

    pub fn remove(&self, path: &str, recursive: bool) -> Result<(), TerminalError> {
        let physical = self.resolve_path(path)?;
        if !physical.exists() {
            return Err(TerminalError::FileNotFound(path.to_string()));
        }
        if physical == self.root_dir {
            return Err(TerminalError::ExecutionError(
                "cannot remove root directory".to_string(),
            ));
        }

        if physical.is_dir() {
            if recursive {
                fs::remove_dir_all(&physical)
                    .map_err(|e| TerminalError::ExecutionError(format!("rmdir error: {}", e)))
            } else {
                fs::remove_dir(&physical)
                    .map_err(|e| TerminalError::ExecutionError(format!("rmdir error: {}", e)))
            }
        } else {
            fs::remove_file(&physical)
                .map_err(|e| TerminalError::ExecutionError(format!("rm error: {}", e)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_path_normalization() {
        let temp = std::env::temp_dir().join("zitera_test_vfs_norm");
        let _ = fs::create_dir_all(&temp);
        let vfs = TerminalFilesystem::new(temp.clone());

        assert_eq!(vfs.normalize_virtual_path("/"), "/");
        assert_eq!(vfs.normalize_virtual_path("/a/b/c"), "/a/b/c");
        assert_eq!(vfs.normalize_virtual_path("/a/../b"), "/b");
        assert_eq!(vfs.normalize_virtual_path("/../../.."), "/");
        assert_eq!(vfs.normalize_virtual_path("/a/./b/./c"), "/a/b/c");

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_escape_attempts_blocked() {
        let temp = std::env::temp_dir().join("zitera_test_vfs_escape");
        let _ = fs::create_dir_all(&temp);
        let vfs = TerminalFilesystem::new(temp.clone());

        // Drive letters
        assert!(matches!(
            vfs.resolve_path("C:/Windows"),
            Err(TerminalError::PathTraversalAttempt(_))
        ));

        // Backslashes
        assert!(matches!(
            vfs.resolve_path("..\\secret"),
            Err(TerminalError::PathTraversalAttempt(_))
        ));

        // UNC
        assert!(matches!(
            vfs.resolve_path("//server/share"),
            Err(TerminalError::PathTraversalAttempt(_))
        ));

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_vfs_file_lifecycle() {
        let temp = std::env::temp_dir().join("zitera_test_vfs_lifecycle");
        let _ = fs::create_dir_all(&temp);
        let mut vfs = TerminalFilesystem::new(temp.clone());

        assert_eq!(vfs.current_virtual_dir(), "/");

        vfs.write_file("test.txt", "hello zitera").unwrap();
        let content = vfs.read_file("test.txt").unwrap();
        assert_eq!(content, "hello zitera");

        vfs.make_dir("subdir").unwrap();
        vfs.change_dir("subdir").unwrap();
        assert_eq!(vfs.current_virtual_dir(), "/subdir");

        vfs.write_file("sub.txt", "nested content").unwrap();
        let sub_content = vfs.read_file("/subdir/sub.txt").unwrap();
        assert_eq!(sub_content, "nested content");

        vfs.change_dir("..").unwrap();
        assert_eq!(vfs.current_virtual_dir(), "/");

        let _ = fs::remove_dir_all(&temp);
    }
}
