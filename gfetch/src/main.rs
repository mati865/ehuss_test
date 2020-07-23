use std::path::Path;
use anyhow::*;

fn main() {
    if let Err(e) = doit() {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}

fn doit() -> Result<(), Error> {
    let repo_path = Path::new("index");
    let mut opts = git2::RepositoryInitOptions::new();
    opts.external_template(false);
    let repo = git2::Repository::init_opts(&repo_path, &opts)?;

    let mut cmd = std::process::Command::new("git");
    cmd.arg("fetch");
    cmd.arg("--force") // handle force pushes
        .arg("--update-head-ok") // see discussion in #2078
        .arg("https://github.com/rust-lang/crates.io-index")
        .arg("HEAD:refs/remotes/origin/HEAD")
        // If cargo is run by git (for example, the `exec` command in `git
        // rebase`), the GIT_DIR is set by git and will point to the wrong
        // location (this takes precedence over the cwd). Make sure this is
        // unset so git will look at cwd for the repo.
        .env_remove("GIT_DIR")
        // The reset of these may not be necessary, but I'm including them
        // just to be extra paranoid and avoid any issues.
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .env_remove("GIT_ALTERNATE_OBJECT_DIRECTORIES")
        .current_dir(repo.path());
    let status = cmd.status()?;
    assert!(status.success());
    Ok(())
}
