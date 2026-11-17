// Integration test for SigmaOS-arch-pacman-mirrors
// Verifies the crate builds and links as a dependency.

use sigmaos_arch_pacman_mirrors::{Mirror, MirrorManager};

#[test]
fn crate_links_and_runs() {
    // Arrange
    let mut mgr = MirrorManager::new();
    mgr.add_mirror(Mirror::new("https://mirror.example.com", "US").with_response_time(50)).unwrap();

    // Act
    let best = mgr.best_mirror();

    // Assert
    assert!(best.is_some());
    assert_eq!(best.unwrap().url, "https://mirror.example.com");
}
