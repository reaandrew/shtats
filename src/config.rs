use clap::ArgMatches;

#[derive(Default)]
pub struct Config {
    pub until: Option<String>,
    pub since: Option<String>,
    pub output: String
}

impl Config{
    pub fn from(app_matches: ArgMatches) -> Config {
        let mut config: Config = Default::default();
        let run_matches = match app_matches.subcommand() {
            Some(("run", matches)) => matches,
            _ => unreachable!("clap should ensure we don't get here"),
        };

        match run_matches
            .value_of_os("until") {
            None => {}
            Some(until) => {
                config.until = Some(String::from(until.to_str().unwrap()))
            }
        };

        match run_matches
            .value_of_os("since") {
            None => {}
            Some(since) => {
                config.since = Some(String::from(since.to_str().unwrap()))
            }
        };

        match run_matches
            .value_of_os("output") {
            None => {}
            Some(output) => {
                config.output = String::from(output.to_str().unwrap())
            }
        };
        config
    }
}