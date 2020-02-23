use std::env;
use std::process::Command;

/// Check if `git` command exists in your environment
pub fn check() -> Result<std::process::Output, std::io::Error> {
    Command::new("git").output()
}

/// Check if there are unstaged changes
pub fn check_unstaged() -> bool {
    String::from_utf8(
        Command::new("git")
            .args(&["status", "-s"])
            .output()
            .expect("Error in checking git status")
            .stdout,
    )
    .expect("Error in getting output")
    .is_empty()
}

/// Get current working directory path string
pub fn current_dir_path() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_owned()
}

/// Get latest commit hash string by using `git log`
pub fn latest_commit_hash(path: &str) -> String {
    String::from_utf8(
        Command::new("git")
            .args(vec!["-C", &path, "log", "-1", "--format=%H"])
            .output()
            .unwrap_or_else(|_| {
                panic!(format!(
                    "Error in executing `git -C {} log` -1 --format=%H",
                    path
                ))
            })
            .stdout,
    )
    .expect("Error in getting output")
    .trim_end()
    .to_owned()
}

/// Get commit object file content using `git cat-file -p <hash>`
pub fn cat_file(path: &str, hash: &str) -> String {
    String::from_utf8(
        Command::new("git")
            .args(vec!["-C", &path, "cat-file", "-p", hash])
            .output()
            .unwrap_or_else(|_| {
                panic!(format!(
                    "Erorr in executing `git -C {} cat-file -p {}`",
                    path, hash
                ))
            })
            .stdout,
    )
    .expect("Error in getting output")
}

/// Change committer name of the commit whose hash is specified.
pub fn filter_branch(path: &str, latest_commit_hash: &str, committer_name: &str) {
    Command::new("git")
        .args(&[
            "-C",
            path,
            "filter-branch",
            "-f",
            "--env-filter",
            &format!(
                r#"if [ "$GIT_COMMIT" = '{}' ]; then export GIT_COMMITTER_NAME='{}'; fi"#,
                latest_commit_hash, committer_name
            ),
            "HEAD^..HEAD", //            "HEAD",
        ])
        .output()
        .expect("err");
}
