use std::env::current_dir;
use std::{env, fs};
use tempdir::TempDir;
use shtats::config::Config;
use shtats::errors::{ErrorType, ShtatsError};
use shtats::html::{HtmlReporter, PreactTemplate};
use shtats::output::BufferedOutput;
use shtats::process::{ProcessGitExecutor, Shtats};
use shtats::Reporter;
use crate::common::{create_file, git_commit, git_init, git_log, git_status};

mod common;

#[test]
#[ignore]
/// This is a really basic, rough test asserting on one fact of the JSON inside the generated HTML
fn test_cli_report() {
    let tmp_dir = TempDir::new("example").expect("could not create the temp directory");
    let path = tmp_dir.path().join("ssgs");
    fs::create_dir(&path).expect("could not create temp directory");

    git_init(&path);
    create_file(&path);
    git_commit(&path, "file 1");
    git_status(&path);
    println!("Committed file 1");
    create_file(&path);
    git_commit(&path, "file 2");
    git_status(&path);
    println!("Committed file 2");
    create_file(&path);
    git_commit(&path, "file 3");
    println!("Committed file 3");
    git_status(&path);
    git_log(&path);

    let mut reporter = HtmlReporter::new( PreactTemplate{});
    let config = Config {
        until: None,
        since: None,
        output: String::from(&path.join("report.html").display().to_string()),
    };
    let executor = Box::new(ProcessGitExecutor {});
    let process_callback = &|| {};
    let mut shtats = Shtats::create(&mut reporter, config, executor, &process_callback);
    shtats.run(&path).expect("error runnning shtats");

    assert!(reporter.to_string().contains("{\"name\":\"Number of commits_collection\",\"value\":\"3\"}"));
}


#[test]
#[ignore]
fn test_capturing_failure_on_non_git_repository(){
    let tmp_dir = TempDir::new("example").expect("could not create the temp directory");
    let path = tmp_dir.path().join("ssgs");
    fs::create_dir(&path).expect("could not create temp directory");

    let _ = env::set_current_dir(&path);

    let mut reporter = HtmlReporter::new(PreactTemplate{});
    let config = Config {
        until: None,
        since: None,
        output: String::from(&path.join("report.html").display().to_string()),
    };
    let executor = Box::new(ProcessGitExecutor {});
    let process_callback = &|| {};
    let mut shtats = Shtats::create(&mut reporter, config, executor, &process_callback);
    let result = shtats.run(current_dir().unwrap().as_path());

    assert!(result.is_err());
    assert!(matches!(result, Err(ShtatsError::Regular(ErrorType::ErrExecutingGit))));
}