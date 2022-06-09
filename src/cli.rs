use clap::{Arg, ArgMatches, Command};
use git_version::git_version;

const GIT_VERSION: &str = git_version!();

pub struct Cli{

}

impl Cli {
    pub fn create(args: Vec<String>) -> ArgMatches {
        let app_matches = Command::new("Shtats")
            .author("Andy Rea, email@andrewrea.co.uk")
            .version(GIT_VERSION)
            .about("Gather statistics from your git repository")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand(Command::new("run").arg(
                Arg::new("until")
                    .long("until")
                    .allow_invalid_utf8(true)
                    .takes_value(true)
                    .help("gather stats on all commits_collection until this date"),
            ).arg(
                Arg::new("since")
                    .long("since")
                    .allow_invalid_utf8(true)
                    .takes_value(true)
                    .help("gather stats on all commits_collection since this date"),
            ).arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .allow_invalid_utf8(true)
                    .takes_value(true)
                    .default_value("report.html")
                    .help("output filename to write the report to"),
            ), )
            .get_matches_from(args);
        app_matches
    }
}