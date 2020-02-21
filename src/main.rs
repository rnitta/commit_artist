mod git;
mod matcher;
mod settings;

use crate::git::commit_object::CommitObject;
use crate::git::git_command;
use crate::matcher::Matcher;
use crate::settings::{MatchMode, Settings};
use crypto::sha1::Sha1;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let path: String = git_command::current_dir_path();
    // TODO: Configurable with Command Line
    let settings = Settings::new(path, MatchMode::Pattern("00000".to_string()), 12);
    //    let settings = Settings::new(path, MatchMode::Repeated(7), 10);

    if git_command::check().is_err() {
        println!("git command not found");
        return;
    }

    if !git_command::check_unstaged() {
        println!(
            "There are unstages changes. You should stash or discard them before running this."
        );
        return;
    }

    let latest_commit_hash = git_command::latest_commit_hash(&settings.path);
    if latest_commit_hash.is_empty() {
        println!("No Commits are Detected.");
        return;
    }

    let latest_cat_file: String = git_command::cat_file(&settings.path, &latest_commit_hash);
    let co = CommitObject::parse_cat_file(&latest_cat_file);
    let new_committer_name = art(settings.clone(), &co, settings.jobs);
    git_command::filter_branch(&settings.path, &latest_commit_hash, &new_committer_name);
    let latest_commit_hash = git_command::latest_commit_hash(&settings.path);
    println!(
        "Yay! Now your new hash of the latest commit is \x1b[31m{}\x1b[m.",
        latest_commit_hash
    );
}

fn art(settings: Settings, commit_object: &CommitObject, job_count: usize) -> String {
    let mut found_hash: String = "".to_owned();
    let mut iteration_count = 0;
    let (tx, rx) = channel();

    while found_hash.is_empty() {
        for i in 0..job_count {
            let settings: Settings = settings.clone();
            let tx = tx.clone();
            let mut co = commit_object.clone();

            thread::spawn(move || {
                let mut hasher = Sha1::new();
                co.committer = {
                    let mut committer = co.committer;
                    committer
                        .name
                        .push_str(&(iteration_count * job_count + i).to_string());
                    committer
                };
                co.to_sha1(&mut hasher);
                let mut commit_hash = co.to_sha1(&mut hasher);

                // FIXME: static dispatch
                let checker: Box<dyn Fn(&str) -> bool> = match settings.mode {
                    MatchMode::Repeated(x) => Box::new(move |s: &str| s.starts_with_repdig(x)),
                    MatchMode::Pattern(x) => Box::new(move |s: &str| s.starts_with(&x)),
                };
                for _ in 0..0xfffff {
                    let mut committer = co.committer.clone();
                    committer.name = commit_hash.clone();
                    co.committer = committer;
                    let pre = commit_hash.clone();
                    commit_hash = co.to_sha1(&mut hasher);
                    if checker(&commit_hash) {
                        tx.send(Some(pre)).unwrap();
                        return;
                    }
                }
                tx.send(None).unwrap();
            });
        }
        for _ in 0..job_count {
            let r = rx.recv().unwrap();
            if let Some(r) = r {
                found_hash = r;
            }
        }
        iteration_count += 1;
        // TODO: print progresses
    }
    found_hash
}