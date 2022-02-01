use std::path::Path;
use forora::{BufferedOutput, run_forora};

fn main() {
    //git rev-list --all --count
    //
    //  The above will give you the number of commits first so a progress bar can be displayed.

    let mut output = BufferedOutput::new();
    match run_forora(Path::new("."), &mut output){
        Ok(_) => {
            println!("{}", output.to_string())
        }
        Err(_) => {
            println!("something went wrong");
        }
    }
}
