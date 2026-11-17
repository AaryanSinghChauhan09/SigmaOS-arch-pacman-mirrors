//! Package mirror management for SigmaOS.
//!
//! Manages mirror lists, ranking by speed, and selection for package downloads.
//! Inspired by Arch Linux's pacman-mirrors but implemented as standalone Rust.

use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

/// Errors for mirror operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirrorError {
    EmptyUrl,
    InvalidUrl,
    MirrorNotFound(String),
    SyncFailed,
}

impl fmt::Display for MirrorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUrl => write!(f, "mirror URL must not be empty"),
            Self::InvalidUrl => write!(f, "invalid mirror URL"),
            Self::MirrorNotFound(m) => write!(f, "mirror not found: {m}"),
            Self::SyncFailed => write!(f, "mirror sync failed"),
        }
    }
}

impl std::error::Error for MirrorError {}

/// Mirror entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Mirror {
    pub url: String,
    pub country: String,
    pub priority: u32,
    pub enabled: bool,
    pub last_sync: Option<u64>,
    pub response_time_ms: Option<u64>,
}

impl Mirror {
    pub fn new(url: impl Into<String>, country: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            country: country.into(),
            priority: 0,
            enabled: true,
            last_sync: None,
            response_time_ms: None,
        }
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_response_time(mut self, ms: u64) -> Self {
        self.response_time_ms = Some(ms);
        self
    }
}

/// Mirror manager.
#[derive(Debug, Clone, Default)]
pub struct MirrorManager {
    mirrors: HashMap<String, Mirror>,
}

impl MirrorManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_mirror(&mut self, mirror: Mirror) -> Result<(), MirrorError> {
        if mirror.url.trim().is_empty() {
            return Err(MirrorError::EmptyUrl);
        }
        if !mirror.url.starts_with("http") {
            return Err(MirrorError::InvalidUrl);
        }
        let url = mirror.url.clone();
        self.mirrors.insert(url, mirror);
        Ok(())
    }

    pub fn remove_mirror(&mut self, url: &str) -> Result<(), MirrorError> {
        self.mirrors.remove(url).ok_or_else(|| MirrorError::MirrorNotFound(url.to_string()))?;
        Ok(())
    }

    pub fn get_mirror(&self, url: &str) -> Option<&Mirror> {
        self.mirrors.get(url)
    }

    pub fn list_mirrors(&self) -> Vec<&Mirror> {
        self.mirrors.values().collect()
    }

    pub fn list_enabled(&self) -> Vec<&Mirror> {
        self.mirrors.values().filter(|m| m.enabled).collect()
    }

    pub fn rank_by_speed(&self) -> Vec<&Mirror> {
        let mut mirrors: Vec<&Mirror> = self.list_enabled();
        mirrors.sort_by_key(|m| m.response_time_ms.unwrap_or(u64::MAX));
        mirrors
    }

    pub fn rank_by_priority(&self) -> Vec<&Mirror> {
        let mut mirrors: Vec<&Mirror> = self.list_enabled();
        mirrors.sort_by_key(|m| m.priority);
        mirrors
    }

    pub fn best_mirror(&self) -> Option<&Mirror> {
        self.rank_by_speed().first().copied()
    }

    pub fn sync_mirror(&mut self, url: &str) -> Result<(), MirrorError> {
        let mirror = self.mirrors.get_mut(url)
            .ok_or_else(|| MirrorError::MirrorNotFound(url.to_string()))?;
        let start = Instant::now();
        // Simulate sync delay
        std::thread::sleep(Duration::from_millis(1));
        mirror.last_sync = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
        mirror.response_time_ms = Some(start.elapsed().as_millis() as u64);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_mirror_aaa() {
        // Arrange
        let mut mgr = MirrorManager::new();
        let mirror = Mirror::new("https://mirror.example.com", "US")
            .with_priority(1)
            .with_response_time(50);

        // Act
        let res = mgr.add_mirror(mirror);

        // Assert
        assert!(res.is_ok());
        assert_eq!(mgr.list_mirrors().len(), 1);
    }

    #[test]
    fn test_invalid_url_rejected_aaa() {
        // Arrange
        let mut mgr = MirrorManager::new();
        let mirror = Mirror::new("ftp://invalid", "US");

        // Act
        let res = mgr.add_mirror(mirror);

        // Assert
        assert!(res.is_err());
    }

    #[test]
    fn test_rank_by_speed_aaa() {
        // Arrange
        let mut mgr = MirrorManager::new();
        mgr.add_mirror(Mirror::new("https://slow.example.com", "US").with_response_time(200)).unwrap();
        mgr.add_mirror(Mirror::new("https://fast.example.com", "DE").with_response_time(20)).unwrap();

        // Act
        let ranked = mgr.rank_by_speed();

        // Assert
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0].url, "https://fast.example.com");
    }

    #[test]
    fn test_sync_mirror_aaa() {
        // Arrange
        let mut mgr = MirrorManager::new();
        mgr.add_mirror(Mirror::new("https://mirror.example.com", "US")).unwrap();

        // Act
        let res = mgr.sync_mirror("https://mirror.example.com");

        // Assert
        assert!(res.is_ok());
        assert!(mgr.get_mirror("https://mirror.example.com").unwrap().last_sync.is_some());
    }
}
