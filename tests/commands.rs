use std::fs;

use rhostman::commands;

fn scratch_hosts_file(initial: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hosts");
    fs::write(&path, initial).unwrap();
    (dir, path)
}

#[test]
fn add_appends_a_new_entry() {
    let (_dir, path) = scratch_hosts_file("127.0.0.1 localhost\n");

    commands::add::handle_command(
        path.clone(),
        "1.2.3.4".to_string(),
        vec!["foo.test".to_string()],
        Some("note".to_string()),
    )
    .unwrap();

    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("1.2.3.4 foo.test # note"));
    assert!(content.contains("127.0.0.1 localhost"));
}

#[test]
fn disable_by_host_comments_out_exactly_one_line() {
    let (_dir, path) = scratch_hosts_file("1.2.3.4 foo.test\n1.2.3.4 keep.test\n");

    commands::disable::handle_command(path.clone(), Some("foo.test".to_string()), None).unwrap();

    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("#rhostman-disabled# 1.2.3.4 foo.test"));
    assert!(content.contains("1.2.3.4 keep.test"));
    assert!(!content.contains("#rhostman-disabled# 1.2.3.4 keep.test"));
}

#[test]
fn disable_by_domain_comments_out_every_matching_subdomain() {
    let (_dir, path) = scratch_hosts_file("0.0.0.0 ads.example.com\n0.0.0.0 tracker.example.com\n1.2.3.4 keep.test\n");

    commands::disable::handle_command(path.clone(), None, Some("example.com".to_string())).unwrap();

    let content = fs::read_to_string(&path).unwrap();
    assert_eq!(content.matches("#rhostman-disabled#").count(), 2);
    assert!(content.contains("1.2.3.4 keep.test"));
}

#[test]
fn remove_by_host_deletes_exactly_one_line() {
    let (_dir, path) = scratch_hosts_file("1.2.3.4 foo.test\n1.2.3.4 keep.test\n");

    commands::remove::handle_command(path.clone(), Some("foo.test".to_string()), None).unwrap();

    let content = fs::read_to_string(&path).unwrap();
    assert!(!content.contains("foo.test"));
    assert!(content.contains("keep.test"));
}

#[test]
fn remove_by_domain_deletes_every_matching_subdomain() {
    let (_dir, path) = scratch_hosts_file("0.0.0.0 ads.example.com\n0.0.0.0 tracker.example.com\n1.2.3.4 keep.test\n");

    commands::remove::handle_command(path.clone(), None, Some("example.com".to_string())).unwrap();

    let content = fs::read_to_string(&path).unwrap();
    assert!(!content.contains("example.com"));
    assert!(content.contains("keep.test"));
}

#[test]
fn remove_requires_exactly_one_selector() {
    let (_dir, path) = scratch_hosts_file("1.2.3.4 foo.test\n");

    assert!(commands::remove::handle_command(path.clone(), None, None).is_err());
    assert!(commands::remove::handle_command(path, Some("foo.test".to_string()), Some("test".to_string())).is_err());
}

#[test]
fn mutating_commands_leave_a_timestamped_auto_backup() {
    let (dir, path) = scratch_hosts_file("1.2.3.4 foo.test\n");

    commands::remove::handle_command(path, Some("foo.test".to_string()), None).unwrap();

    let backups: Vec<_> = fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().contains("rhostman.bak"))
        .collect();
    assert_eq!(backups.len(), 1);
}

#[test]
fn backup_copies_current_content_to_output_path() {
    let (dir, path) = scratch_hosts_file("1.2.3.4 foo.test\n");
    let output = dir.path().join("hosts.bak");

    commands::backup::handle_command(path, output.clone()).unwrap();

    let content = fs::read_to_string(&output).unwrap();
    assert_eq!(content, "1.2.3.4 foo.test\n");
}
