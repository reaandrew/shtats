use std::path::Path;
use forora::{BufferedOutput, HtmlReporter, Reporter, run_forora};

fn main() {
    //git rev-list --all --count
    //
    //  The above will give you the number of commits first so a progress bar can be displayed.

    let mut output = BufferedOutput::new();
    let reporter = HtmlReporter::new();
    match run_forora(Path::new("."), &mut output, Box::new(reporter)){
        Ok(_) => {
            println!("{}", output.to_string())
        }
        Err(_) => {
            println!("something went wrong");
        }
    }
}
