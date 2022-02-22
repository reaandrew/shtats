use std::path::Path;
use shtats::html::HtmlReporter;
use shtats::output::BufferedOutput;
use shtats::process::run_shtats;

fn main() {
    //git rev-list --all --count
    //
    //  The above will give you the number of commits first so a progress bar can be displayed.

    // TODO: Duplicate Commit Messages
    // TODO: Duplicate Commit Messages by user
    // TODO: Commits By Year if no time filter has been applied

    let mut output = BufferedOutput::new();
    let reporter = HtmlReporter::new();
    match run_shtats(Path::new("."), &mut output, Box::new(reporter)) {
        Ok(_) => {
            println!("{}", output.to_string())
        }
        Err(_) => {
            println!("something went wrong");
        }
    }
}

#[cfg(test)]
mod tests {
    use shtats::duplicates::DuplicateDetector;
    #[test]
    fn test_something() {
        let data = vec!["blue
            green
            red
            purple", "blue
            green
            red
            purple","red
            purple"];


        let mut dup_detector = DuplicateDetector::new(3);
        for bunch in data {
            let lines = bunch.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
            dup_detector.add(lines);
        }

        for item in dup_detector.results(){
            println!("BING {}: {}", item.count, item.duplicate)
        }
    }
}
