use rustic_backend::BackendOptions;
use rustic_core::{BackupOptions, PathList, Repository, RepositoryOptions, SnapshotOptions};
use std::error::Error;

pub fn snapshot(repository: &str, password: &str, paths: Vec<&str>) -> Result<(), Box<dyn Error>> {
    // Initialize Backends
    let backends = BackendOptions::default()
        .repository(repository)
        .to_backends()?;

    println!("successfully initialized backends:\n{backends:#?}");

    // Open repository
    let repo_opts = RepositoryOptions::default().password(password);

    let repo = Repository::new(&repo_opts, backends)?
        .open()?
        .to_indexed_ids()?;

    println!("successfully opened repository:\n{repo:#?}");

    let backup_opts = BackupOptions::default();
    let source = PathList::from_strings(paths).sanitize()?;

    println!("successfully sanitized paths:\n{source:#?}");

    let snap = SnapshotOptions::default().to_snapshot()?;

    println!("successfully created snapshot options:\n{snap:#?}");

    // Create snapshot
    let snap = repo.backup(&backup_opts, &source, snap)?;

    println!("successfully created snapshot:\n{snap:#?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot() {
        let repository = "/tmp/test";
        let password = "password";
        let paths = vec!["/etc"];

        assert!(snapshot(repository, password, paths).is_ok());
    }
}
