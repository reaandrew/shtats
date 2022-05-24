use std::fs::File;
use std::io::{BufReader, Error, Write};
use std::path::Path;
use std::process::{ChildStdout, Command, Stdio};
use serde_json::{ Value};
use crate::{GitStat, GitStatsJsonViewModel, Reporter};
use crate::collectors::commits_by_day::CommitsByDayCollector;
use crate::collectors::commits_by_file_extension::CommitsByFileExtension;
use crate::collectors::files_by_commits::FilesByCommitsCollector;
use crate::collectors::files_by_day::FilesByDayCollector;
use crate::collectors::files_by_lines::FilesByLines;
use crate::collectors::lines_by_day::LinesByDayCollector;
use crate::collectors::messages::MessagesCollector;
use crate::collectors::punch_card::PunchCardCollector;
use crate::collectors::summary_stats::SummaryStatsCollector;
use crate::collectors::user_summary::UserSummaryCollector;
use crate::config::Config;
use crate::parsers::{GitCommitIterator, StdoutGitLogReader};

pub struct Shtats<'a, 'b> {
    reporter: Box<&'a mut dyn Reporter>,
    config: Config,
    process_callback: Box<&'b dyn Fn() -> ()>
}

impl Shtats<'_, '_>{

    pub fn create<'a, 'b>(reporter: &'a mut dyn Reporter, config: Config, process_callback: &'b dyn Fn() -> ()) -> Shtats<'a, 'b> {
        return Shtats{
            reporter: Box::new(reporter),
            config,
            process_callback: Box::new(process_callback)
        }
    }

    pub fn run(&mut self, path: &Path) -> Result<(), Error> {
        let mut stats_functions = self.create_stat_collectors();
        let args = create_git_log_args(&self.config);

        let stdout = execute_git(args, path);
        let buf_reader = BufReader::new(stdout);
        let reader = StdoutGitLogReader{stdout: buf_reader};
        let iterator = GitCommitIterator::new(Box::new(reader));

        for commit in iterator{
            for stat in &mut stats_functions{
                stat.process(&commit);
            }
            (self.process_callback)();
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
        self.reporter.write(viewmodel);

        let path = Path::new(&self.config.output);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(self.reporter.to_string().as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => { }
        }

        Ok(())
    }

    fn create_stat_collectors(&self) -> Vec<Box<dyn GitStat>> {
        let stats_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector::default()),
            Box::new(CommitsByDayCollector::default()),
            Box::new(LinesByDayCollector::default()),
            Box::new(MessagesCollector::default()),
            //Box::new(SimilarFilesChangingCollector::default()),
            Box::new(FilesByDayCollector::default()),
            Box::new(PunchCardCollector::default()),
            Box::new(FilesByCommitsCollector::default()),
            Box::new(FilesByLines::default()),
            Box::new(CommitsByFileExtension::default()),
            Box::new(UserSummaryCollector::default())
        ];
        stats_functions
    }
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