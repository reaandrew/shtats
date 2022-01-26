use std::path::Path;
use forora::run_forora;

fn main() {
    //git rev-list --all --count
    //
    //  The above will give you the number of commits first so a progress bar can be displayed.

    match run_forora(Path::new(".")){
        Ok(_) => {}
        Err(_) => {
            println!("something went wrong");
        }
    }
}
