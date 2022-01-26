use std::{fs, path};
use std::fs::File;
use std::io::{Error, Write};
use std::path::{PathBuf};
use std::process::{Command, Stdio};
use tempdir::TempDir;
use uuid::Uuid;


fn git_init(path: &PathBuf) -> Result<(), Error> {
    let mut git_init = Command::new("git");
    git_init.args(vec!["init"]);
    git_init.current_dir(
        &path
    ).output()?;
    Ok(())
}

fn git_status(path: &PathBuf) -> Result<(), Error> {
    let mut git_status = Command::new("git");
    git_status.args(vec!["status"]);
    git_status.stdout(Stdio::piped());
    let output = git_status.current_dir(
        &path
    ).output()?;

    println!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(())
}

fn git_commit(path: &PathBuf, message: &str) -> Result<(), Error>{
    let mut git_add = Command::new("git");
    git_add.args(vec![
        "add",
        "-A",
    ]);
    git_add.stdout(Stdio::piped());
    git_add.current_dir(
        &path
    ).output()?;

    let mut git_commit = Command::new("git");
    git_commit.args(vec![
        "commit",
        "-a",
        "-m",
        message
    ]);
    git_commit.stdout(Stdio::piped());
    let output = git_commit.current_dir(
        &path
    ).output()?;

    println!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(())
}

fn git_log(path: &PathBuf) -> Result<(), Error>{
    let mut git_log = Command::new("git");
    git_log.args(vec![
        "log",
        "--oneline"
    ]);
    git_log.stdout(Stdio::piped());
    let output = git_log.current_dir(
        &path
    ).output()?;
    println!("{}", String::from_utf8(output.stdout).unwrap());
    Ok(())
}

fn create_file(path: &PathBuf) -> Result<(), Error>{
    let uuid = Uuid::new_v4();
    let mut file = File::create(path.join(uuid.to_string()))?;
    file.write_all(b"Hello, world!")?;

    Ok(())
}

pub fn setup_git_repo<T>(test_func: T) -> Result<(), Error> where T: Fn(&path::Path) -> Result<(), Error>{
    let tmp_dir = TempDir::new("example").expect("could not create the temp directory");
    let path = tmp_dir.path().join("ssgs");

    fs::create_dir(&path).expect("could not create temp directory");

    let run = || -> Result<(), Error>{
        git_init(&path)?;
        create_file(&path)?;
        git_commit(&path, "file 1")?;
        create_file(&path)?;
        git_commit(&path, "file 2")?;
        create_file(&path)?;
        git_commit(&path, "file 3")?;
        git_status(&path)?;
        git_log(&path)?;
        Ok(())
    };

    match run() {
        Ok(_) => {
            test_func(path.as_path())?;
            tmp_dir.close()?;
            Ok(())
        }
        Err(err) => {
            println!("{}", err);
            tmp_dir.close()?;
            Err(err)
        }
    }
}
