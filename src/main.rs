mod external_command;
mod git;
mod settings;

use crate::external_command as command;
use crate::git::commit_object::CommitObject;
use crate::settings::Settings;
use crypto::sha1::Sha1;
use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name("Commit Artist")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("commit_artist <flags>")
        .action(art)
        .flags(vec![
            Flag::new(
                "path",
                "[optional] --path <path_to_your_repository>",
                FlagType::String,
            ),
            Flag::new(
                "pattern",
                "[optional] --pattern <[0-9a-f]{1,40}>",
                FlagType::String,
            )
            .alias("p"),
            Flag::new("block", "[optional] --block 28", FlagType::Int).alias("b"),
            Flag::new("jobs", "[optional] --jobs 4", FlagType::Int).alias("j"),
        ]);

    app.run(args);
}

/// as you see
fn art(c: &Context) {
    let mut settings = Settings::default();

    if let Some(path) = c.string_flag("path") {
        settings.path = path;
    }

    if let Some(pattern) = c.string_flag("pattern") {
        settings.pattern(pattern);
    }

    if let Some(block) = c.int_flag("block") {
        settings.block_size(block as usize);
    }

    if let Some(jobs) = c.int_flag("jobs") {
        settings.jobs(jobs as usize);
    }

    if command::check().is_err() {
        println!("git command not found");
        return;
    }

    if !command::check_unstaged() {
        println!(
            "There are unstages changes. You should stash or discard them before running this."
        );
        return;
    }

    let latest_commit_hash = command::latest_commit_hash(&settings.path);
    if latest_commit_hash.is_empty() {
        println!("No Commits are Detected.");
        return;
    }

    let latest_cat_file: String = command::cat_file(&settings.path, &latest_commit_hash);
    let co = CommitObject::parse_cat_file(&latest_cat_file);
    let new_committer_name = bruteforce(settings.clone(), &co, settings.jobs);
    command::filter_branch(&settings.path, &latest_commit_hash, &new_committer_name);
    let latest_commit_hash = command::latest_commit_hash(&settings.path);
    println!(
        "Yay! Now your new hash of the latest commit is \x1b[31m{}\x1b[m.",
        latest_commit_hash
    );
}

/// Spawn bruteforce thread and catch the result and check it and loop back unless there are no expected result.
fn bruteforce(settings: Settings, commit_object: &CommitObject, job_count: usize) -> String {
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
