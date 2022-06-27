use std::fs::File;
use std::io::{ BufReader, Read, Write};
use std::path::Path;
use std::process::{ChildStdout, Command, Stdio};
use serde_json::{Value};
use crate::collectors::commits_by_day::CommitsByDayCollector;
use crate::collectors::commits_by_file_extension::CommitsByFileExtension;
use crate::collectors::files_by_commits::FilesByCommitsCollector;
use crate::collectors::files_by_day::FilesByDayCollector;
use crate::collectors::files_by_lines::FilesByLines;
use crate::collectors::lines_by_average::LinesByAverageCollector;
use crate::collectors::lines_by_day::LinesByDayCollector;
use crate::collectors::messages::MessagesCollector;
use crate::collectors::punch_card::PunchCardCollector;
use crate::collectors::summary_stats::SummaryStatsCollector;
use crate::collectors::user_summary::UserSummaryCollector;
use crate::config::Config;
use crate::errors::{ErrorType, ShtatsError};
use crate::parsers::{GitCommitIterator, StdoutGitLogReader};
use crate::reporter::Reporter;
use crate::result;
use crate::stats::{GitStat, LineStatsAverage};
use crate::viewmodel::GitStatsJsonViewModel;


pub trait GitExecutor {
    fn execute(&self, args: Vec<String>, path: &Path) -> result::Result<GitCommitIterator>;
}

pub struct ProcessGitExecutor {}

impl ProcessGitExecutor {
    fn check_status(&self, path: &Path) -> result::Result<()> {
        let child = Command::new("git")
            .current_dir(path)
            .args(vec!["status"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().unwrap();
        let stderr = child.stderr.unwrap();
        let mut buf_reader = BufReader::new(stderr);
        let mut output: String = String::new();

        let read_size = buf_reader.read_to_string(&mut output).unwrap();

        if read_size != 0 {
            Err(ShtatsError::Regular(ErrorType::ErrExecutingGit))
        }else{
            Ok(())
        }
    }

    fn execute_git(&self, args: Vec<String>, path: &Path) -> result::Result<ChildStdout> {
        self.check_status(path)?;

        let child = Command::new("git")
            .current_dir(path)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()?;
        //.expect("Could not spawn git process.")
        let stdout = child.stdout.unwrap();
        //.expect("Could not capture standard output.");
        Ok(stdout)
    }
}

impl GitExecutor for ProcessGitExecutor {
    fn execute(&self, args: Vec<String>, path: &Path) -> result::Result<GitCommitIterator> {
        let stdout = self.execute_git(args, path)?;
        let buf_reader = BufReader::new(stdout);
        let reader = StdoutGitLogReader { stdout: buf_reader };
        Ok(GitCommitIterator::new(Box::new(reader)))
    }
}

pub struct Shtats<'a, 'b> {
    reporter: Box<&'a mut dyn Reporter>,
    config: Config,
    process_callback: Box<&'b dyn Fn() -> ()>,
    git_executor: Box<dyn GitExecutor>,
}

impl Shtats<'_, '_> {
    pub fn create<'a, 'b>(reporter: &'a mut dyn Reporter,
                          config: Config,
                          git_executor: Box<dyn GitExecutor>,
                          process_callback: &'b dyn Fn() -> ()) -> Shtats<'a, 'b> {
        return Shtats {
            reporter: Box::new(reporter),
            config,
            git_executor,
            process_callback: Box::new(process_callback),
        };
    }

    pub fn run(&mut self, path: &Path) -> result::Result<()> {
        let mut stats_functions = self.create_stat_collectors();
        let args = create_git_log_args(&self.config);

        let iterator = self.git_executor.execute(args, path)?;

        for commit in iterator {
            for stat in &mut stats_functions {
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

        let report_path = Path::new(&self.config.output);
        let display = path.display();

        let mut file = match File::create(&report_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(self.reporter.to_string().as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => {}
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
            Box::new(UserSummaryCollector::default()),
            Box::new(LinesByAverageCollector::default())
        ];
        stats_functions
    }
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

pub fn get_number_of_commits() -> result::Result<i32>{
    let output = Command::new("git")
        .args(["rev-list", "--all", "--count"])
        .output()
        .expect("failed to execute process");

    if !output.status.success(){
        let data = String::from_utf8(output.stderr).unwrap();

        if data.contains("unsafe repository"){
            Err(ShtatsError::Regular(ErrorType::ErrUnsafeGitRepository))
        }else if data.contains("not a git repository") {
            Err(ShtatsError::Regular(ErrorType::ErrNotGitRepository))
        }else {
            Err(ShtatsError::Regular(ErrorType::ErrExecutingGit))
        }
    }else {
        Ok(String::from_utf8(output.stdout).unwrap().trim().parse()
            .expect("unexpected output from git revlist --all --count and could not parse."))
    }
}

#[cfg(test)]
mod tests{
    use crate::process::create_git_log_args;

    #[test]
    fn test_git_log_args(){
        let expected = vec![
            "--no-pager",
            "log",
            "--all",
            "--raw",
            "--decorate",
            "--date-order",
            "--reverse",
            "--numstat",
            "--date=rfc2822",
        ];
        let actual = create_git_log_args(&Default::default());

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod stderr_tests {
    use std::io::{BufReader, Read};
    use std::path::Path;
    use std::process::{Command, Stdio};

    #[test]
    fn test_something() {
        let child = Command::new("git")
            .current_dir(Path::new("./"))
            .args(vec!["bla"])
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().unwrap();

        assert!(child.stderr.is_some());
        let stderr = child.stderr.unwrap();
        let mut buf_reader = BufReader::new(stderr);
        let mut output: String = String::new();
        let read_size = buf_reader.read_to_string(&mut output).unwrap();
        assert!(read_size > 0)
    }
}
