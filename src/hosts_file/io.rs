use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use nix::unistd::{chown, Gid, Uid};

use crate::errors::RhostmanResult;
use crate::hosts_file::document::HostsDocument;

pub fn read(path: &Path) -> RhostmanResult<HostsDocument> {
    let content = fs::read_to_string(path)?;
    Ok(HostsDocument::parse(&content))
}

/// Writes `doc` to `path` by rendering to a temp file in the same
/// directory (so the final rename is atomic on the same filesystem),
/// copying the original file's permissions/ownership, then renaming over
/// the target. Cleans up the temp file on any failure.
pub fn write_atomic(path: &Path, doc: &HostsDocument) -> RhostmanResult<()> {
    let dir = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let tmp_path = temp_path_in(dir, path);

    let result: RhostmanResult<()> = (|| {
        fs::write(&tmp_path, doc.render())?;
        if let Ok(metadata) = fs::metadata(path) {
            fs::set_permissions(&tmp_path, metadata.permissions())?;
            let _ = chown(
                &tmp_path,
                Some(Uid::from_raw(metadata.uid())),
                Some(Gid::from_raw(metadata.gid())),
            );
        }
        fs::rename(&tmp_path, path)?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&tmp_path);
    }
    result
}

fn temp_path_in(dir: &Path, target: &Path) -> PathBuf {
    let file_name = target.file_name().and_then(|n| n.to_str()).unwrap_or("hosts");
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.join(format!(".{}.rhostman-tmp.{}", file_name, nanos))
}

pub fn backup_path_for(target: &Path) -> PathBuf {
    let dir = target
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let file_name = target.file_name().and_then(|n| n.to_str()).unwrap_or("hosts");
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    dir.join(format!("{}.rhostman.bak.{}", file_name, ts))
}

/// Copies `target` to a timestamped backup path next to it, before a
/// mutating command proceeds. Separate from the user-invoked `backup`
/// subcommand, which writes to a user-chosen output path instead.
pub fn auto_backup(target: &Path) -> RhostmanResult<PathBuf> {
    let backup_path = backup_path_for(target);
    fs::copy(target, &backup_path)?;
    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hosts_file::document::HostsDocument;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn write_atomic_round_trips_content_and_permissions() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("hosts");
        fs::write(&path, "127.0.0.1 localhost\n").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o640)).unwrap();

        let mut doc = read(&path).unwrap();
        doc.add_entry("1.2.3.4", &["foo.test".to_string()], None).unwrap();
        write_atomic(&path, &doc).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("foo.test"));

        let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o640);

        let leftovers: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains("rhostman-tmp"))
            .collect();
        assert!(
            leftovers.is_empty(),
            "temp file should not remain after a successful write"
        );
    }

    #[test]
    fn auto_backup_copies_current_content() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("hosts");
        fs::write(&path, "127.0.0.1 localhost\n").unwrap();

        let backup_path = auto_backup(&path).unwrap();
        let backed_up = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backed_up, "127.0.0.1 localhost\n");
    }

    #[test]
    fn read_parses_into_document() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("hosts");
        fs::write(&path, "127.0.0.1 localhost\n").unwrap();
        let doc: HostsDocument = read(&path).unwrap();
        assert!(doc.find_entry_by_host("localhost").is_some());
    }
}
