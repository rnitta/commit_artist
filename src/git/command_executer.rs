use std::env;
use std::process::Command;

// check if `git` command is executable or not
pub fn check() -> Result<std::process::Output, std::io::Error> {
    Command::new("git").output()
}

// check if there are unstaged changes
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

// get working directory path string
pub fn current_dir_path() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_owned()
}

// get latest commit hash using external command `git log`
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

// get commit object file content using `git cat-file -p <hash>`
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
            //"HEAD^..HEAD"
            "HEAD",
        ])
        .output()
        .expect("err");
}
