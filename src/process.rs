use std::fs::File;
use std::io::{BufReader, Error, Write};
use std::path::Path;
use std::process::{ChildStdout, Command, Stdio};
use serde_json::{ Value};
use crate::{create_stat_collectors, GitStatsJsonViewModel, Reporter};
use crate::parsers::{GitCommitIterator, StdoutGitLogReader};

#[derive(Default)]
pub struct Config {
    pub until: Option<String>,
    pub since: Option<String>,
    pub output: String
}


// TODO: Change design to reduce number of arguments.
pub fn run_shtats(path: &Path, reporter: &mut dyn Reporter, config: Config, process_callback: &dyn Fn() -> ()) -> Result<(), Error> {
    let mut stats_functions = create_stat_collectors();
    let args = create_git_log_args(&config);
    let stdout = execute_git(args, path);

    let buf_reader = BufReader::new(stdout);
    let reader = StdoutGitLogReader{stdout: buf_reader};
    let iterator = GitCommitIterator::new(Box::new(reader));

    for commit in iterator{
        for stat in &mut stats_functions{
            stat.process(&commit);
        }
        process_callback();
    }

    let mut viewmodel = GitStatsJsonViewModel::default();
    for stat in stats_functions.iter() {
        let json_viewmodel = stat.get_json_viewmodel().unwrap();
        let summaries = json_viewmodel.summary.iter()
            .map(|x| {
                return serde_json::to_value(x).unwrap();
            }).collect::<Vec<Value>>();
        viewmodel.summary.extend(summaries);
        viewmodel.data.insert(json_viewmodel.key, json_viewmodel.data);
    }
    reporter.write(viewmodel);

    let path = Path::new(&config.output);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(reporter.to_string().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => { }
    }

    Ok(())
}


fn execute_git(args: Vec<String>, path: &Path) -> ChildStdout {
    let stdout = Command::new("git")
        .current_dir(path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not spawn git process.")
        .stdout
        .expect("Could not capture standard output.");
    stdout
}

pub fn get_number_of_commits() -> i32{
    let output = Command::new("git")
        .args(["rev-list","--all","--count"])
        .output()
        .expect("failed to execute process");

    return String::from_utf8(output.stdout).unwrap().trim().parse()
        .expect("unexpected output from git revlist --all --count and could not parse.");
}

fn create_git_log_args(config: &Config) -> Vec<String> {
    let mut args = vec![
        String::from("--no-pager"),
        String::from("log"),
        String::from("--all"),
        String::from("--raw"),
        String::from("--decorate"),
        String::from("--date-order"),
        String::from("--reverse"),
        String::from("--numstat"),
        String::from("--date=rfc2822"),
    ];
    match &config.until {
        None => {}
        Some(until) => {
            args.push(String::from("--until"));
            args.push(String::from(until));
        }
    }
    match &config.since {
        None => {}
        Some(since) => {
            args.push(String::from("--since"));
            args.push(String::from(since));
        }
    }
    args
}