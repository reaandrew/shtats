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

#[cfg(test)]
mod tests{
    use crate::cli;
    use crate::config::Config;

    fn get_args(args: Vec<&str>) -> Vec<String>{
         let mut run_args = vec![
            "shtats",
            "run",
        ];
        run_args.extend(args);
        return run_args.iter().map(|&x|String::from(x)).collect::<Vec<String>>();
    }

    #[test]
    fn test_something(){
        let args = get_args(vec![
            "-o",
            "/etc/somewhere"
        ]);
        let matches = cli::Cli::create(args);
        let config = Config::from(matches);
        assert_eq!(config.output, "/etc/somewhere");
    }

    #[test]
    fn test_since(){
        let args = get_args(vec![
            "--since",
            "1 month ago"
        ]);
        let matches = cli::Cli::create(args);
        let config = Config::from(matches);
        assert_eq!(config.since, Some(String::from("1 month ago")));
    }

    #[test]
    fn test_until(){
        let args = get_args(vec![
            "--until",
            "1 month ago"
        ]);
        let matches = cli::Cli::create(args);
        let config = Config::from(matches);
        assert_eq!(config.until, Some(String::from("1 month ago")));
    }
}