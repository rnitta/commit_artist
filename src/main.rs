mod git;
mod settings;

use crate::git::commit_object::CommitObject;
use crate::git::git_command;
use crate::settings::Settings;
use crypto::sha1::Sha1;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let mut settings = Settings::default();
    settings.pattern = "0000000".to_owned();

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
    println!();

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

                for _ in 0..1u64 << settings.block_size {
                    co.committer.name = commit_hash.clone();
                    let pre = commit_hash.clone();
                    commit_hash = co.to_sha1(&mut hasher);
                    if commit_hash.starts_with(&settings.pattern) {
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
        println!(
            "\x1b[1A{} hashes calculated...",
            iteration_count as u128 * (1 << settings.block_size) as u128 * settings.jobs as u128
        );
    }
    found_hash
}
