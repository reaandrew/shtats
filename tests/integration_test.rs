use std::{env, fs};
use std::io::Error;
use std::path::Path;
use std::process::{Command, Stdio};

#[test]
fn test_cli_report() {
    let mut dir = env::temp_dir();
    let path = dir.join("ssgs");
    let setup = || -> Result<(), Error > {

        fs::create_dir(&path)?;

        let mut git_status = Command::new("git");
        git_status.args(vec!["status"]);
        git_status.stdout(Stdio::piped());
        let output = git_status.current_dir(
            &path
        ).output().expect("error executing git status");

        let std_err_contents = String::from_utf8(output.stderr).unwrap();
        let std_out_contents = String::from_utf8(output.stdout).unwrap();
        Ok(())
    };

    setup();
    fs::remove_dir_all(&path);
}