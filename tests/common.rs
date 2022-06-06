use std::fs::File;
use std::io::{Write};
use std::path::{PathBuf};
use std::process::{Command, Stdio};
use uuid::Uuid;


pub fn git_init(path: &PathBuf) {
    let mut git_init = Command::new("git");
    git_init.args(vec!["init"]);
    git_init.current_dir(
        &path
    ).output().expect("failed to git init");
}

pub fn git_status(path: &PathBuf){
    let mut git_status = Command::new("git");
    git_status.args(vec!["status"]);
    git_status.stdout(Stdio::piped());
    let _output = git_status.current_dir(
        &path
    ).output().expect("failed to execute git status");
}

pub fn git_commit(path: &PathBuf, message: &str){
    let mut git_add = Command::new("git");
    git_add.args(vec![
        "add",
        "-A",
    ]);
    git_add.stdout(Stdio::piped());
    git_add.current_dir(
        &path
    ).output().expect("failed to execute git add");

    let mut git_commit = Command::new("git");
    git_commit.args(vec![
        "commit",
        "-a",
        "-m",
        message
    ]);
    git_commit.stdout(Stdio::piped());
    let _output = git_commit.current_dir(
        &path
    ).output().expect("failed to execute git commit");
}

pub fn git_log(path: &PathBuf){
    let mut git_log = Command::new("git");
    git_log.args(vec![
        "log",
        "--oneline"
    ]);
    git_log.stdout(Stdio::piped());
    let _output = git_log.current_dir(
        &path
    ).output().expect("failed to execute git log");
}

pub fn create_file(path: &PathBuf){
    let uuid = Uuid::new_v4();
    let mut file = File::create(path.join(uuid.to_string())).expect("failed to create file");
    file.write_all(b"Hello, world!").expect("failed to write file");
}

